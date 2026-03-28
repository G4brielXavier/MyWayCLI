// NATIVE
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// by MYWAY
use crate::cli::commands::Commands;
use crate::log::log::{ Log, LogF };

use crate::core::project::{ Project, GenericList, User };
use crate::core::filemanager::Fiman;
use crate::core::errors::MyWayError;
use crate::core::filemanager::ReturnReadType;

// CRATES
use chrono::{ Local };
use colored::Colorize;
use tequel_rs::rng::TequelRng;

use semver::Version;
use reqwest::header::USER_AGENT;



pub fn check_for_updates(log: &Log) -> Option<String> {
    let current_version = env!("CARGO_PKG_VERSION");
    let url = "https://crates.io/api/v1/crates/myway-cli";

    let client = reqwest::blocking::Client::new()
    ;
    let response = client
        .get(url)
        .timeout(Duration::from_secs(2))
        .header(USER_AGENT, "myway-cli-updater (contact: dotxavket@gmail.com)")
        .send()
        .ok()?;

    let json : serde_json::Value = response.json().ok()?;

    if let Some(latest_version_str) = json["crate"]["max_version"].as_str() {
        let current = Version::parse(current_version).ok()?;
        let latest = Version::parse(latest_version_str).ok()?;

        if latest > current {
            log.hey_mw(&format!("Hey! This MyWay is from older version, update now!"));
            log.hey_mw(&format!("{} -> {}", current, latest));
            log.hey_mw(&format!("Use: {}", "cargo install myway-cli".yellow()));
        } else {
            log.hey_mw(&format!("MyWay is {}!", "updated".yellow()));
        }
    }

    None

}



pub fn match_cli(command: &Commands, log: Log, files: &mut Fiman, data: &mut GenericList, graveyard: &mut GenericList, userdata: &mut User) -> Result<(), MyWayError> {

    let teq_rng: TequelRng = TequelRng::new();

    match command {

        // Verify "myway hey"
        Commands::Hey { update } => {
            
            if *update {
                check_for_updates(&log);
            } else {
                log.ascii_myway();
            }

            Ok(())
        }
        
        
        // Verify "myway create"
        // Create and Add a new project to way
        Commands::Add => {

            let mut uuid_project_val = teq_rng.rand_deep_string(3);

            if data.iter().any(|p| p.uuid == uuid_project_val) {
                uuid_project_val = teq_rng.rand_deep_string(3);
            }

            let msg_uuid = format!("uuid_project: {}", uuid_project_val.bold());            
            let proj_name_def: String = format!("my_project_{}", uuid_project_val);
            
            log.hey_mw("Preparing to create project...");
            log.hey("");
            log.hey_mw(&format!("UUID Generated with {}", "Tequel".bold()));
            log.hey(&msg_uuid);
            log.hey("");
            
            let mut proj_name = log.quest_mandatory("project_name", &proj_name_def);

            if proj_name.len() > 20 {
                return Err(MyWayError::StringLengthLimitExceeded("project_name: <=20 limit length".to_string()))
            }

            if proj_name.contains(char::is_whitespace) {
                let old_name = proj_name.clone();
                proj_name = proj_name.replace(" ", "-");
                log.hey_mw(&format!("\"{}\" converted to \"{}\"", old_name, &proj_name));
            }

            if data.iter().any(|p| p.name == proj_name) {
                log.hey("");
                return Err(MyWayError::ProjectAlreadyExists(format!("\"{}\" already exist as project", proj_name)))
            }

            let proj_description: String = log.quest_mandatory("description", "It's my project");
            
            log.hey("");
            log.hey_mw(&format!("{} Use \"SemVer\" as '0.1.0'. No spaces.", "Recommended:".yellow()));
            let mut proj_version: String = log.quest_mandatory("version", "0.1.0");

            if proj_version.len() > 8 {
                return Err(MyWayError::StringLengthLimitExceeded("version: <=8 limit length".to_string()))
            }

            if proj_version.contains(char::is_whitespace) {
                let old_name = proj_version.clone();
                proj_version = proj_version.replace(" ", ".");
                log.hey_mw(&format!("\"{}\" converted to \"{}\"", old_name, &proj_version));
            }

            log.hey("");
            log.hey_mw(&format!("{}", "Stack is what you are using to creating your project.".dimmed()));
            log.hey_mw(&format!("{} [react-ts, zustand, node, express]", "Example:".yellow()));
            let proj_stack: String = log.quest_mandatory("stack/tools", "[]");
            
            let clean_input = proj_stack.trim_matches(|c| c == '[' || c == ']');            

            let res: Vec<String> = clean_input.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

            log.hey("");
            let proj_mission: String = log.quest_option("YOU WILL FINISH THAT? [Y|n]", vec!["Y", "n"], "Y");
            
            log.hey("");
            
            let creating_msg = format!("Adding {} to your WAY... ", proj_name.yellow().bold());
            let final_msg = format!("Now {} is on your WAY!", proj_name.yellow().bold());
            
            log.hey_mw(&creating_msg);
            thread::sleep(Duration::from_millis(500));
            log.hey_mw(&final_msg);

            let now = Local::now();
            let format_data = now.format("%Y-%m-%d %H:%M:%S");

            let my_project: Project = Project {
                uuid: uuid_project_val,

                name: proj_name,
                description: proj_description,
                stack: res,

                your_think: "".to_string(),

                mission: proj_mission,
                
                versions: vec![proj_version],
                status: "new".to_string(),
                
                time_created: format_data.to_string(),

                trace: vec![],

                is_finish: false
            };

            data.push(my_project);
            userdata.projects_created += 1;
            
            files.write(&ReturnReadType::User(userdata.clone()), &files.user_path.clone())?;
            files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;
            
            Ok(())

        }
        

        // Verify "myway edit --uuid --name"
        // Create and Add a new project to way
        Commands::Edit { uuid, name  } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                data.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                data.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };

            if let Some(uuid) = target_uuid {

                if let Some(proj) = data.iter().find(|p| p.uuid == uuid) {
                    
                    log.hey_mw(&format!("Prepairing to edit \"{}\"...", proj.name));
                    log.hey("");
                    
                    let mut proj_name = log.quest_mandatory("project_name", &proj.name);
    
                    if proj_name.len() > 20 {
                        return Err(MyWayError::StringLengthLimitExceeded("project_name: <=20 limit length".to_string()))
                    }

                    if proj_name.contains(char::is_whitespace) {
                        let old_name = proj_name.clone();
                        proj_name = proj_name.replace(" ", "-");
                        log.hey_mw(&format!("\"{}\" converted to \"{}\"", old_name, &proj_name));
                    }
    
                    // if data.iter().any(|p| p.name == proj_name) {
                    //     log.hey("");
                    //     log.hey_mw(&format!("This {} already exist with name: {}", "project_name".bold(), proj_name));
                    //     std::process::exit(1)
                    // }
    
                    let proj_description: String = log.quest_mandatory("description", &proj.description);

                    log.hey("");
                    log.hey_mw(&format!("{}", "Stack is what you are using to creating your project.".dimmed()));
                    log.hey_mw(&format!("{} [react-ts, zustand, node, express]", "Example:".yellow()));
    
                    let display_stacks = format!("[{}]", proj.stack.join(", "));        
                    let proj_stack: String = log.quest_mandatory("stack/tools", &display_stacks);
                    
                    let clean_input = proj_stack.trim_matches(|c| c == '[' || c == ']');            
                    let res: Vec<String> = clean_input.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    
                    log.hey("");
                    let proj_mission: String = log.quest_option("YOU WILL FINISH THAT? [Y|n]", vec!["Y", "n"], &proj.mission);
                    
                    log.hey("");
                    
                    log.hey_mw(&format!("{}", "Editted with successfully!".yellow()));
                    let creating_msg = format!("{}", "Updating your WAY... ".dimmed());
                    let final_msg = format!("Now {} is updated on your WAY!", proj_name.yellow().bold());
                    
                    log.hey_mw(&creating_msg);
                    thread::sleep(Duration::from_millis(500));
                    log.hey_mw(&final_msg);
    
                    let my_project: Project = Project {
                        uuid: proj.uuid.clone(),
    
                        name: proj_name,
                        description: proj_description,
                        stack: res,
    
                        your_think: "".to_string(),
    
                        mission: proj_mission,
                        
                        versions: proj.versions.clone(),
                        status: proj.status.clone(),
                        
                        time_created: proj.time_created.clone(),
                        trace: vec![],
    
                        is_finish: proj.is_finish.clone()
                    };
    
                    if let Some(proj) = data.iter_mut().find(|s| s.uuid == my_project.uuid) {
                        *proj = my_project;
                    }
    
                    files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;

                    Ok(())

                } else {
                    log.hey_mw("Project not exist or not found.");
                    Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()))
                }
                
            } else {
                log.hey_mw(&format!("{} Use '--uuid' or '--name' and verify if the project exist.", "Tip:".yellow()));
                return Err(MyWayError::InvalidInput("You don't provided a identifier or the project not exist".to_string()))
            }


        }
        

        // Verify "myway way"
        // List all projects on way
        Commands::Way { oneline, complex, uuid, name, finish, working, status } => {
            
            log.hey_mw(&format!("{}", "Walking on Way!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));
    
            if let Some(id) = uuid {

                match data.iter().find(|p| p.uuid == *id) {
                    Some(project) => {
                        println!("");

                        let is_finished = if project.is_finish {
                            "F"
                        } else {
                            "W"
                        };

                        log.hey_mw(&format!("({} : {}) {} {} {}", is_finished.bold().yellow(), project.status.yellow().bold(), project.uuid.italic(), project.name.bold(), format!("[{}]", project.versions.last().expect("")).italic()));

                        if !*oneline {
                            if *complex { log.hey_project(project, true); } 
                            else { log.hey_project(project, false); }
                        } else {
                            if *complex { log.hey_project(project, true); }
                        }

                        println!("");

                        Ok(())
                    }
                    None => {
                        Err(MyWayError::ProjectNotFound(format!("\"{}\" not exist or not found", id)))
                    }
                }
                    
            } else if let Some(name) = name {

                match data.iter().find(|n| n.name == *name) {
                    Some(project) => {
                        println!("");

                        let is_finished = if project.is_finish {
                            "F"
                        } else {
                            "W"
                        };

                        log.hey_mw(&format!("({} : {}) {} {} {}", is_finished.bold().yellow(), project.status.yellow().bold(), project.uuid.italic(), project.name.bold(), format!("[{}]", project.versions.last().expect("")).italic()));

                        if !*oneline {
                            if *complex { log.hey_project(project, true); } 
                            else { log.hey_project(project, false); }
                        } else {
                            if *complex { log.hey_project(project, true); }
                        }

                        println!("");
                        
                        Ok(())


                    }
                    None => {
                        Err(MyWayError::ProjectNotFound(format!("\"{}\" not exist or not found", name)))
                    }
                }

            } else {

                let mut project_way = data.clone();

                if let Some(status) = status {

                    project_way = project_way.iter().filter(|p| p.status == *status).cloned().collect();

                } else {

                    if *finish {
                        project_way = project_way.iter().filter(|p| p.is_finish == true).cloned().collect();
                    }

                    if *working {
                        project_way = project_way.iter().filter(|p| p.is_finish == false).cloned().collect();
                    }

                }

                log.hey_mw(&format!("{} project(s) found", project_way.len()));
                log.hey(&format!("{}", ">-----------------------------".dimmed()));
                log.hey("");

                for project in project_way.iter() {

                    let is_finished = if project.is_finish {
                        "F"
                    } else {
                        "W"
                    };

                    log.hey_mw(&format!("({} : {}) {} {} {}", is_finished.bold().yellow(), project.status.yellow().bold(), project.uuid.italic(), project.name.bold(), format!("[{}]", project.versions.last().expect("")).italic()));

                    if !*oneline {
                        if *complex { log.hey_project(project, true); } 
                        else { log.hey_project(project, false); }
                    } else {
                        if *complex { log.hey_project(project, true); }
                    }

                    println!("");

                }

                Ok(())

            }

        }
        
        
        // Verify "myway giveup"
        // Remove a project from WAY
        Commands::Giveup { uuid, name } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                data.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                data.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };

            if let Some(uuid) = target_uuid {

                if let Some(project) = data.iter().find(|p| p.uuid == *uuid) {
    
                    let uuid_removed = project.uuid.clone();
        
                    let is_finished = if project.is_finish {
                        "F"
                    } else {
                        "W"
                    };

                    log.hey_mw(&format!("You will {} this:", "Giveup".red()));
                    log.hey(&format!("{}", ">-----------------------------".dimmed()));
                    log.hey("");
                    log.hey_mw(&format!("({} : {}) {} {} {}", is_finished.bold().yellow(), project.status.yellow().bold(), project.uuid.italic(), project.name.bold(), format!("[{}]", project.versions.last().expect("")).italic()));
                    log.hey_project(project, true);
                    
                    log.hey("");
    
                    let want_giveup = log.quest_option("Proceed? [Y/n]: ", vec!["Y", "n"], "n");
    
                    if want_giveup == "Y" {
    
                        log.hey_mw(&format!("{} was removed!", project.name.yellow()));
                        
                        data.retain(|p| p.uuid != uuid_removed);

                        
                        userdata.projects_giveup += 1;

                        files.write(&ReturnReadType::User(userdata.clone()), &files.user_path.clone())?;
    
                        files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;

                        Ok(())
    
                    } else {
                        Ok(())
                    }

                } else {
                    Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()))
                }

            } else {
                log.hey_mw(&format!("{} Use '--uuid' or '--name' and verify if the project exist.", "Tip:".yellow()));
                return Err(MyWayError::InvalidInput("You don't provided a identifier or the project not exist".to_string()))
            }
        
        }
        
        
        // Verify "myway finish"
        // Mark a project as FINISHED!
        Commands::Finish { uuid, name } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                data.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                data.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };


            if let Some(uuid) = target_uuid {

                if let Some(proj) = data.iter().find(|p| p.uuid == *uuid) {

                    log.hey_mw(&format!("{}", "This action cannot be revoked!".dimmed().red()));
                    println!("");

                    let want_finish = log.quest_option("Did you really mark this project as \"Finish\"? [Y/n]:", vec!["Y", "n"], "Y");

                    if want_finish == "Y" {

                        log.hey_mw(&format!("{} was marked as ({})!", proj.name.yellow(), "F".bold().yellow()));
                        
                        let my_project: Project = Project {
                            uuid: proj.uuid.clone(),
                            name: proj.name.clone(),
                            description: proj.description.clone(),
                            stack: proj.stack.clone(),
                            versions: proj.versions.clone(),
                            your_think: proj.your_think.clone(),
                            mission: proj.mission.clone(),
                            status: proj.status.clone(),
                            is_finish: true,
                            trace: proj.trace.clone(),
                            time_created: proj.time_created.clone()
                        };


                        if let Some(proj) = data.iter_mut().find(|p| p.uuid == my_project.uuid) {
                            *proj = my_project;
                        }

                        
                        userdata.projects_finish += 1;

                        files.write(&ReturnReadType::User(userdata.clone()), &files.user_path.clone())?;

                        files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;

                        Ok(())

                    } else {
                        Ok(())
                    }

                } else {
                    Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()))
                }

            } else {
                log.hey_mw(&format!("{} Use '--uuid' or '--name' and verify if the project exist.", "Tip:".yellow()));
                return Err(MyWayError::InvalidInput("You don't provided a identifier or the project not exist".to_string()))
            }

        }



        // Verify "myway stacks"
        Commands::Stacks => {

            log.hey_mw("Searching all Stacks...");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let mut counts: HashMap<String, u32> = HashMap::new();

            for proj in data.iter() {
                for stack in proj.stack.iter() {
                    *counts.entry(stack.clone()).or_insert(0) += 1;
                }
            }   

            let mut display_stacks: Vec<String> = counts
                .into_iter()
                .map(|(name, count)| {
                    if count > 1 {
                        format!("{} (x{})", name, count)
                    } else {
                        name
                    }
                })
                .collect();

            display_stacks.sort();

            if !display_stacks.is_empty() {
                log.hey_mw(&format!("{} Stack(s) Mentioned:", display_stacks.len()));
                log.hey("");

                for proj_stack in display_stacks.iter() {
                    log.hey(&format!("- {}", proj_stack.bold()));
                }

                Ok(())
            } else {
                Err(MyWayError::StacksIsEmpty(format!("Stacks is empty")))
            }


        }




        Commands::Version { uuid, name, list, add } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                data.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                data.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };

            if *add {

                if let Some(uuid) = target_uuid {

                    if let Some(proj) = data.iter_mut().find(|p| p.uuid == *uuid) {

                        let proj_name = proj.name.clone();
                        let proj_ver = proj.versions.clone();
                        let curr_ver = proj_ver.last().expect("");
                        
                        log.hey_mw(&format!("\"{}\" is on {} currently", proj_name, curr_ver));
                        log.hey("");

                        let proj_version_new = log.quest_mandatory("New Version", "").trim().to_string();

                        if proj_version_new.is_empty() {
                            log.hey("");
                            return Err(MyWayError::InvalidInput("Your input is empty".to_string()));
                        }

                        if proj_version_new.len() > 8 {
                            return Err(MyWayError::StringLengthLimitExceeded("version: <=8 limit length".to_string()))
                        }

                        if proj.versions.iter().any(|v| *v == *proj_version_new) {
                            log.hey("");
                            return Err(MyWayError::VersionAlreadyExists(format!("\"{}\" already exists a version.", proj_version_new)))
                        }

                        
                        log.hey_mw(&format!("{} -> {}", curr_ver, proj_version_new));
                        log.hey_mw(&format!("{} has a new version!", proj.name.bold()));
                        proj.versions.push(proj_version_new);
                        files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;

                        return Ok(());


                    } else {
                        return Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()));
                    }

                } else {
                    log.hey_mw(&format!("{} Use '--uuid' or '--name' and verify if the project exist.", "Tip:".yellow()));
                    return Err(MyWayError::InvalidInput("You don't provided a identifier or the project not exist".to_string()))
                }

            }

            if *list {

                if let Some(uuid) = target_uuid {

                    if let Some(proj) = data.iter().find(|p| p.uuid == uuid) {

                        let project_name = proj.name.clone();
                        let project_versions = proj.versions.clone();

                        log.hey_mw(&format!("On \"{}\"'s versions", project_name));
                        log.hey_mw(&format!("{} version(s) found", project_versions.len()));
                        log.hey(">-----------------------------");
                        log.hey("");

                        for version in project_versions.iter() {

                            println!("- {}", version);

                        }   

                    } else {
                        return Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()));
                    }

                } else {
                    log.hey_mw(&format!("{} Use '--uuid' or '--name' and verify if the project exist.", "Tip:".yellow()));
                    return Err(MyWayError::InvalidInput("You don't provided a identifier or the project not exist".to_string()))
                }

            }

            Ok(())

        }




        Commands::Yard { uuid, name, list, kill, exject } => {

            log.hey_mw(&format!("{}", "On Graveyard!".red()));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                data.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                data.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };

            if *kill {

                if let Some(uuid) = target_uuid {

                    if let Some(proj) = data.iter_mut().find(|p| p.uuid == *uuid) {

                        let proj = proj.clone();

                        log.hey_mw(&format!("{}", "You are about to kill a project".red().italic()));
                        
                        let want = log.quest_option(&format!("KILL {}? [Y/n]", proj.name.yellow().bold()), vec!["Y", "n"], "n").trim().to_string();

                        if want.is_empty() {
                            return Err(MyWayError::InvalidInput("Input is Empty".to_string()))
                        } 

                        if want == "Y" {

                            log.hey("");
                            log.hey_mw(&format!("{} is {}", proj.name.yellow().bold(), "Dead!".red()));

                            data.retain(|p| p.uuid != &*proj.uuid);
                            graveyard.push(proj);

                            userdata.projects_killed += 1;

                            files.write(&ReturnReadType::User(userdata.clone()), &files.user_path.clone())?;
                            files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;
                            files.write(&ReturnReadType::GenericList(graveyard.to_vec()), &files.graveyard_path.clone())?;

                        } else {
                            log.hey_mw("Aborted. Project is still alive.");
                            return Ok(());
                        }

                    } else {
                        return Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()));
                    }

                    
                } else {
                    log.hey_mw("Use '--uuid' or '--name'");
                    return Err(MyWayError::InvalidInput("You don't provided a identifier as uuid or name.".to_string()));
                }

            }

            if *list {

                log.hey_mw(&format!("{} grave(s) found", graveyard.len()));
                log.hey(&format!("{}", ">-----------------------------".dimmed()));
                log.hey("");
                
                for proj in graveyard.iter() {
                    println!("- {} {} {}", proj.uuid.italic(), proj.name.bold(), format!("[{}]", proj.versions.last().expect("")).italic())
                }
                
                log.hey("");
            }

            if *exject {
                
                log.hey_mw("This action will clear all project's that is on Graveyard.");
                let want = log.quest_option("Do \"Exject\"? [Y/n]", vec!["Y", "n"], "n").trim().to_string();

                if want.is_empty() {
                    return Err(MyWayError::InvalidInput("Input is Empty".to_string()))
                } 

                if want == "Y" {

                    log.hey("");
                    log.hey_mw("Graveyard is Empty now!");
                    graveyard.clear();
                    files.write(&ReturnReadType::GenericList(graveyard.to_vec()), &files.graveyard_path.clone())?;


                } else {
                    return Ok(())
                }

            }

            Ok(())

        }




        Commands::Reviv { uuid, name } => {

            log.hey_mw(&format!("{}", "In Th3 0th3r S1d3!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                graveyard.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                graveyard.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };


            if let Some(uuid) = target_uuid {

                if let Some(proj) = graveyard.iter().find(|p| p.uuid == *uuid).cloned() {

                    log.hey_mw(&format!("{}", "You're about to REVIVE a project from Graveyard.".dimmed().yellow()));
                    println!("");


                    let want = log.quest_option(&format!("REVIVE {}? [Y/n]", proj.name.yellow()), vec!["Y", "n"], "n").trim().to_string();

                    if want.is_empty() {
                        return Err(MyWayError::InvalidInput("Input is Empty".to_string()))
                    } 

                    if want == "Y" {

                        log.hey_mw(&format!("{} was revived!", proj.name.yellow()));
                        graveyard.retain(|p| p.uuid != &*proj.uuid);
                        data.push(proj);

                        
                        userdata.projects_revived += 1;

                        files.write(&ReturnReadType::User(userdata.clone()), &files.user_path.clone())?;

                        files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;
                        files.write(&ReturnReadType::GenericList(graveyard.to_vec()), &files.graveyard_path.clone())?;

                    } else {
                        log.hey_mw("Aborted. Project is still dead.");
                    }


                    Ok(())

                } else {
                    Err(MyWayError::ProjectNotFound("Project not found in GRAVEYARD".to_string()))
                }

            } else {
                log.hey_mw(&format!("{} Use '--uuid' or '--name' and verify if the project exist.", "Tip:".yellow()));
                Err(MyWayError::InvalidInput("You don't provided a identifier or the project not exist".to_string()))
            }
        }




        Commands::Status { uuid, name } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                data.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                data.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };


            if let Some(uuid) = target_uuid {

                if let Some(proj) = data.iter().find(|p| p.uuid == *uuid) {

                    log.hey_mw(&format!("{}'s current status is {}", proj.name, proj.status.bold()));
                    println!("");

                    let new_status = log.quest_mandatory("New Status", "stable").trim().to_string().to_lowercase();

                    if !new_status.is_empty() {

                        log.hey_mw(&format!("{}'s status is {} now!", proj.name, new_status.bold().yellow()));
                        
                        let my_project: Project = Project {
                            uuid: proj.uuid.clone(),
                            name: proj.name.clone(),
                            description: proj.description.clone(),
                            stack: proj.stack.clone(),
                            versions: proj.versions.clone(),
                            your_think: proj.your_think.clone(),
                            mission: proj.mission.clone(),
                            status: new_status,
                            trace: proj.trace.clone(),
                            is_finish: proj.is_finish.clone(),
                            time_created: proj.time_created.clone()
                        };


                        if let Some(proj) = data.iter_mut().find(|p| p.uuid == my_project.uuid) {
                            *proj = my_project;
                        }

                        files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;

                        Ok(())

                    } else {
                        log.hey("");
                        Err(MyWayError::InvalidInput("Your input is empty".to_string()))
                    }

                } else {
                    Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()))
                }

            } else {
                log.hey_mw("Use '--uuid' or '--name'");
                Err(MyWayError::InvalidInput("You don't provided a identifier as uuid or name.".to_string()))
            }

        }




        Commands::Ord { uuid, name, first, last, swap } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                data.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                data.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };

            if let Some(uuid) = target_uuid {

                if let Some(proj) = data.iter().find(|p| p.uuid == *uuid) {

                    let proj_name = proj.name.clone();

                    let project_index = data.iter().position(|p| p == proj).ok_or_else(|| {
                        MyWayError::WayLengthExceeded("".to_string())
                    })?;
                    
                    if let Some(name) = swap {

                        let other_project = data.iter().find(|p| p.name == *name).ok_or_else(|| {
                            MyWayError::ProjectNotFound("".to_string())
                        })?;

                        let other_project_index = data.iter().position(|p| p == other_project).ok_or_else(|| {
                            MyWayError::ProjectNotFound("".to_string())
                        })?;

                        if other_project_index < data.len() {

                            log.hey_mw(&format!("{} and {} were swapped", proj_name.yellow(), other_project.name.yellow()));
                            data.swap(project_index, other_project_index);

                            files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;

                        } else {
                            return Err(MyWayError::WayLengthExceeded(format!("{} is more than {}", other_project_index, data.len())))
                        }

                    } else {

                        if *first {

                            if project_index < data.len() {

                                log.hey_mw(&format!("Now, {} is in FIRST place!", proj_name.yellow()));

                                let it = data.remove(project_index);
                                data.insert(0, it);

                                files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;
                            }
                            
                        }
                        
                        if *last {
                            
                            if project_index < data.len() {

                                log.hey_mw(&format!("Now, {} is in last place!", proj_name.yellow()));

                                let it = data.remove(project_index);
                                data.insert(data.len(), it);
                                files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;
                            }

                        }

                    }

                    Ok(())

                } else {
                    Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()))
                }

            } else {
                log.hey_mw("Use '--uuid' or '--name'");
                Err(MyWayError::InvalidInput("You don't provided a identifier as uuid or name.".to_string()))
            }

        }




        Commands::Trace { uuid, name, list, add, remove } => {
            
            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");
            log.hey(&format!("{}", ">-----------------------------".dimmed()));

            let target_uuid = if let Some(u) = uuid {
                data.iter().find(|p| p.uuid == *u).map(|p| p.uuid.clone())
            } else if let Some(n) = name {
                data.iter().find(|p| p.name == *n).map(|p| p.uuid.clone())
            } else {
                None
            };

            if let Some(uuid) = target_uuid {

                if let Some(index) = data.iter().position(|p| p.uuid == uuid) {

                    if *list {
                        for (idx, t) in data[index].trace.iter().enumerate() {
                            log.hey_mw(&format!("{} {} {}", idx.to_string().dimmed(), "--".dimmed(), t))
                        }
                    }

                    if *add {

                        let new_trace = log.quest_mandatory("Note", "I added a new feature").trim().to_string().to_lowercase();

                        let mut proj = data[index].clone();
                        proj.trace.reverse();

                        if !new_trace.is_empty() {
                            
                            let mut my_project: Project = Project {
                                uuid: proj.uuid.clone(),
                                name: proj.name.clone(),
                                description: proj.description.clone(),
                                stack: proj.stack.clone(),
                                versions: proj.versions.clone(),
                                your_think: proj.your_think.clone(),
                                mission: proj.mission.clone(),
                                status: new_trace.clone(),
                                trace: proj.trace.clone(),
                                is_finish: proj.is_finish.clone(),
                                time_created: proj.time_created.clone()
                            };

                            my_project.trace.push(new_trace);

                            if let Some(proj) = data.iter_mut().find(|p| p.uuid == my_project.uuid) {
                                *proj = my_project;
                            }

                            let proj_name = data[index].name.clone();


                            files.write(&ReturnReadType::GenericList(data.to_vec()), &&files.mw_path.clone())?;
                            log.hey_mw(&format!("A New Trace was added to {}", proj_name.yellow()));

                        } else {
                            log.hey("");
                            return Err(MyWayError::InvalidInput("Your input is empty".to_string()))
                        }
                    
                    }

                    if let Some(idx) = remove {

                        let mut proj = data[index].clone();
                        proj.trace.remove(*idx);
                        let proj_name = proj.name.clone();

                        data[index] = proj;

                        files.write(&ReturnReadType::GenericList(data.to_vec()), &files.mw_path.clone())?;

                        log.hey_mw(&format!("A note was removed from {}", proj_name.yellow()));

                    }


                } else {
                    return Err(MyWayError::ProjectNotFound("Project not found.".into()))
                }

            } else {
                log.hey_mw("Use '--uuid' or '--name'");
                return Err(MyWayError::InvalidInput("No identifier provided.".into()))
            }   

            Ok(())


        }


        

        Commands::Stats {  } => {

            let h_user = serde_json::to_value(&userdata).map_err(|e| MyWayError::IoError(e.into()))?; 
            let map : HashMap<String, serde_json::Value> = serde_json::from_value(h_user).unwrap();

            log.hey_mw("Your stats:");
            log.hey("");

            for (k, v) in map.iter() {
                log.hey_mw(&format!("> {}: {}", k.dimmed(), v.to_string().yellow()));
            }

            log.hey("");

            Ok(())
        
        }

    }

}
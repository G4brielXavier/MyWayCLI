use std::collections::HashMap;
// NATIVE
use std::thread;
use std::time::Duration;

// by MYWAY
use crate::cli::commands::Commands;
use crate::log::log::{ Log, LogF };
use crate::core::project::{ GraveyardList, Project, ProjectList, view_mission };
use crate::core::filemanager::Fiman;
use crate::core::errors::MyWayError;

// CRATES
use chrono::{ Local };
use colored::Colorize;
use tequel_rs::rng::TequelRng;

pub fn match_cli(command: &Commands, log: Log, files: &mut Fiman, data: &mut ProjectList, graveyard: &mut GraveyardList) -> Result<(), MyWayError> {

    let teq_rng: TequelRng = TequelRng::new();

    match command {

        // Verify "myway hey"
        Commands::Hey => {
            log.ascii_myway();
            Ok(())
        }
        
        
        // Verify "myway create"
        // Create and Add a new project to way
        Commands::Create => {

            let mut uuid_project_val = teq_rng.rand_deep_string(3);

            if data.iter().any(|p| p.uuid == uuid_project_val) {
                uuid_project_val = teq_rng.rand_deep_string(3);
            }

            let msg_uuid = format!("uuid_project: {}", uuid_project_val.bold());

            let proj_name_def: String = format!("my_project_{}", uuid_project_val);

            log.hey_mw("Preparing to create project...");
            log.hey(&msg_uuid);
            log.hey("");
            
            let mut proj_name = log.quest_mandatory("project_name", &proj_name_def);

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
            log.hey_mw("Tip: Use \"SemVer\" as '0.1.0'");
            let proj_version: String = log.quest_mandatory("version", "0.1.0");

            log.hey("");
            log.hey_mw("Stack is what you are using to creating your project.");
            log.hey_mw("Syntax Example: [react-ts, zustand, node, express]");
            let proj_stack: String = log.quest_mandatory("stack/tools", "[]");
            
            let clean_input = proj_stack.trim_matches(|c| c == '[' || c == ']');            

            let res: Vec<String> = clean_input.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();

            log.hey("");
            log.hey_mw("Tip: \"IDK\" means \"I don't Know\".");
            let proj_mission: String = log.quest_option("YOU WILL FINISH THAT? [Y|idk|n]", vec!["Y", "idk", "n"], "Y");
            
            log.hey("");
            
            let creating_msg = format!("Adding your project > \"{}\" to your WAY... ", proj_name);
            let final_msg = format!("Now \"{}\" is on your WAY!", proj_name);
            
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

                is_finish: false
            };

            data.push(my_project);
            files.write(&data).expect("Not possible to save data");

            Ok(())

        }
        

        // Verify "myway edit --uuid --name"
        // Create and Add a new project to way
        Commands::Edit { uuid, name  } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");

            let mut target_uuid = None;

            if let Some(uuid) = uuid {
                target_uuid = data.iter().find(|p| p.uuid == *uuid).map(|p| p.uuid.clone());
            }

            if let Some(name) = name {
                target_uuid = data.iter().find(|p| p.name == *name).map(|p| p.uuid.clone());
            }


            if let Some(uuid) = target_uuid {

                if let Some(proj) = data.iter().find(|p| p.uuid == uuid) {
                    
                    log.hey_mw(&format!("Prepairing to edit \"{}\"...", proj.name));
                    log.hey("");
                    
                    let mut proj_name = log.quest_mandatory("project_name", &proj.name);
    
                    if proj_name.contains(char::is_whitespace) {
                        proj_name = proj_name.replace(" ", "-");
                    }
    
                    // if data.iter().any(|p| p.name == proj_name) {
                    //     log.hey("");
                    //     log.hey_mw(&format!("This {} already exist with name: {}", "project_name".bold(), proj_name));
                    //     std::process::exit(1)
                    // }
    
                    let proj_description: String = log.quest_mandatory("description", &proj.description);
    
                    log.hey("");
                    log.hey_mw("Stack is what you are using to creating your project.");
                    log.hey_mw("Syntax Example: [react-ts, zustand, node, express]");
    
                    let display_stacks = format!("[{}]", proj.stack.join(", "));        
                    let proj_stack: String = log.quest_mandatory("stack/tools", &display_stacks);
                    
                    let clean_input = proj_stack.trim_matches(|c| c == '[' || c == ']');            
                    let res: Vec<String> = clean_input.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    
                    log.hey("");
                    log.hey_mw("Tip: \"IDK\" means \"I don't Know\".");
                    let proj_mission: String = log.quest_option("YOU WILL FINISH THAT? [Y|idk|n]", vec!["Y", "idk", "n"], &proj.mission);
                    
                    log.hey("");
                    
                    log.hey_mw("Editted with successfully!");
                    let creating_msg = format!("Updating your WAY... ");
                    let final_msg = format!("Now \"{}\" is updated on your WAY!", proj_name);
                    
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
                        status: "new".to_string(),
                        
                        time_created: proj.time_created.clone(),
    
                        is_finish: false
                    };
    
                    if let Some(proj) = data.iter_mut().find(|s| s.uuid == my_project.uuid) {
                        *proj = my_project;
                    }
    
                    files.write(data).expect("Not possible to save data");

                    Ok(())

                } else {
                    log.hey_mw("Project not exist or not found.");
                    Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()))
                }
                
            } else {
                log.hey_mw("Use '--uuid' or '--name'");
                Err(MyWayError::InvalidInput("You don't provided a identifier as uuid or name.".to_string()))
            }


        }
        

        // Verify "myway way"
        // List all projects on way
        Commands::Way { oneline, complex, uuid, name } => {
            
            log.hey_mw(&format!("{}", "Walking on WAY..."));
            log.hey_mw(&format!("{} project(s) found", data.len()));
            log.hey(&format!("{}", ">-----------------------------".dimmed()));
            log.hey("");

            if let Some(id) = uuid {

                match data.iter().find(|p| p.uuid == *id) {
                    Some(project) => {

                        let is_finished = if project.is_finish {
                            "F"
                        } else {
                            "W"
                        };

                        log.hey_mw(&format!("({}) {} {} {}", is_finished.bold().yellow(), project.uuid.italic(), project.name.bold(), format!("[{}]", project.versions.last().expect("")).italic()));

                        if !*oneline {
                            if *complex {

                                println!("\t{}", project.description.italic());
                                println!("\tStatus: {}", project.status.bold());
                                println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
                                println!("\tStack(s):");
                                if project.stack.len() != 0 {
                                    for i in project.stack.iter() {
                                        println!("\t- {} ", i.bold())
                                    }
                                } else {
                                    println!("\t {}", "No Stack(s)".bold())
                                }
                                println!("\tCreated at: {}", project.time_created.italic());

                            } else {
                                
                                println!("\t{}", project.description.italic());
                                println!("\tStatus: {}", project.status.bold());
                                println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
                                println!("\tStack(s):");
                                if project.stack.len() != 0 {
                                    for i in project.stack.iter() {
                                        println!("\t- {} ", i.bold())
                                    }
                                } else {
                                    println!("\t {}", "No Stack(s)".bold())
                                }

                            }
                        } else {
                            if *complex {
                                println!("\t{}", project.description.italic());
                                println!("\tStatus: {}", project.status.bold());
                                println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
                                println!("\tStack(s):");
                                if project.stack.len() != 0 {
                                    for i in project.stack.iter() {
                                        println!("\t- {} ", i.bold())
                                    }
                                } else {
                                    println!("\t {}", "No Stack(s)".bold())
                                }
                                println!("\tCreated at: {}", project.time_created.italic());
                            }
                        }

                        Ok(())
                    }
                    None => {
                        Err(MyWayError::ProjectNotFound(format!("\"{}\" not exist or not found", id)))
                    }
                }
                    
            } else if let Some(name) = name {

                match data.iter().find(|n| n.name == *name) {
                    Some(project) => {
                        
                        let is_finished = if project.is_finish {
                            "F"
                        } else {
                            "W"
                        };

                        log.hey_mw(&format!("({}) {} {} {}", is_finished.bold().yellow(), project.uuid.italic(), project.name.bold(), format!("[{}]", project.versions.last().expect("")).italic()));

                        if !*oneline {
                            if *complex {

                                println!("\t{}", project.description.italic());
                                println!("\tStatus: {}", project.status.bold());
                                println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
                                println!("\tStack(s):");
                                if project.stack.len() != 0 {
                                    for i in project.stack.iter() {
                                        println!("\t- {} ", i.bold())
                                    }
                                } else {
                                    println!("\t {}", "No Stack(s)".bold())
                                }
                                println!("\tCreated at: {}", project.time_created.italic());

                            } else {
                                
                                println!("\t{}", project.description.italic());
                                println!("\tStatus: {}", project.status.bold());
                                println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
                                println!("\tStack(s):");
                                if project.stack.len() != 0 {
                                    for i in project.stack.iter() {
                                        println!("\t- {} ", i.bold())
                                    }
                                } else {
                                    println!("\t {}", "No Stack(s)".bold())
                                }

                            }
                        } else {
                            if *complex {
                                println!("\t{}", project.description.italic());
                                println!("\tStatus: {}", project.status.bold());
                                println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
                                println!("\tStack(s):");
                                if project.stack.len() != 0 {
                                    for i in project.stack.iter() {
                                        println!("\t- {} ", i.bold())
                                    }
                                } else {
                                    println!("\t {}", "No Stack(s)".bold())
                                }
                                println!("\tCreated at: {}", project.time_created.italic());
                            }
                        }

                        Ok(())

                    }
                    None => {
                        Err(MyWayError::ProjectNotFound(format!("\"{}\" not exist or not found", name)))
                    }
                }

            } else {

                for project in data.iter() {

                    let is_finished = if project.is_finish {
                        "F"
                    } else {
                        "W"
                    };

                    log.hey_mw(&format!("({}) {} {} {}", is_finished.bold().yellow(), project.uuid.italic(), project.name.bold(), format!("[{}]", project.versions.last().expect("")).italic()));

                    if !*oneline {
                        if *complex {

                            println!("\t{}", project.description.italic());
                            println!("\tStatus: {}", project.status.bold());
                            println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
                            println!("\tCreated at: {}", project.time_created.italic());
                            println!("\tStack(s):");
                            if project.stack.len() != 0 {
                                for i in project.stack.iter() {
                                    println!("\t- {} ", i.bold())
                                }
                            } else {
                                println!("\t  {}", "No Stack(s)".bold())
                            }

                        } else {
                            
                            println!("\t{}", project.description.italic());
                            println!("\tStatus: {}", project.status.bold());
                            println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());

                        }
                    } else {
                        if *complex {
                            println!("\t{}", project.description.italic());
                            println!("\tStatus: {}", project.status.bold());
                            println!("\tMission: \"{}\"", view_mission(&project.mission).italic().yellow());
                            println!("\tCreated at: {}", project.time_created.italic());
                            println!("\tStack(s):");
                            if project.stack.len() != 0 {
                                for i in project.stack.iter() {
                                    println!("\t- {} ", i.bold())
                                }
                            } else {
                                println!("\t  {}", "No Stack(s)".bold())
                            }
                        }
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

            let mut target_uuid = None;

            if let Some(uuid) = uuid {
                target_uuid = data.iter().find(|p| p.uuid == *uuid).map(|p| p.uuid.clone());
            }

            if let Some(name) = name {
                target_uuid = data.iter().find(|p| p.name == *name).map(|p| p.uuid.clone());
            }

            if let Some(uuid) = target_uuid {

                if let Some(project) = data.iter().find(|p| p.uuid == *uuid) {
    
                    let id_remove = project.uuid.clone();
                    let name_to_log = project.name.clone();
                    let mission_log = project.mission.clone();
    
    
                    println!("");
                    log.hey_mw("You will delete this:");
                    log.hey_mw(&format!("\t#{} {}", id_remove.italic(), name_to_log.bold()));
                        println!("\t\tMission: \"{}\"", view_mission(&mission_log).italic());
    
                    println!("");
    
                    let want_giveup = log.quest_option("Proceed? [Y/n]: ", vec!["Y", "n"], "n");
    
                    if want_giveup == "Y" {
    
                        log.hey_mw(&format!("\"{}\" was removed!", name_to_log));
                        
                        data.retain(|p| p.uuid != id_remove);
    
                        files.write(&data).expect("Not possible to save data");

                        Ok(())
    
                    } else {
                        Ok(())
                    }

                } else {
                    Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()))
                }

            } else {
                log.hey_mw("Use '--uuid' or '--name'");
                Err(MyWayError::InvalidInput("You don't provided a identifier as uuid or name.".to_string()))
            }
        
        }
        
        
        // Verify "myway finish"
        // Mark a project as FINISHED!
        Commands::Finish { uuid, name } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");

            let mut target_uuid = None;

            if let Some(uuid) = uuid {
                target_uuid = data.iter().find(|p| p.uuid == *uuid).map(|p| p.uuid.clone());
            }

            if let Some(name) = name {
                target_uuid = data.iter().find(|p| p.name == *name).map(|p| p.uuid.clone());
            }


            if let Some(uuid) = target_uuid {

                if let Some(proj) = data.iter().find(|p| p.uuid == *uuid) {

                    log.hey_mw(&format!("{}", "This action cannot be revoked!".dimmed().red()));
                    println!("");

                    let want_finish = log.quest_option("Did you really finish the project? [Y/n]:", vec!["Y", "n"], "Y");

                    if want_finish == "Y" {

                        log.hey_mw(&format!("\"{}\" was marked as ({})!", proj.name, "F".bold().yellow()));
                        
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
                            time_created: proj.time_created.clone()
                        };


                        if let Some(proj) = data.iter_mut().find(|p| p.uuid == my_project.uuid) {
                            *proj = my_project;
                        }

                        files.write(&data).expect("Not possible to save data");

                        Ok(())

                    } else {
                        Ok(())
                    }

                } else {
                    Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()))
                }

            } else {
                log.hey_mw("Use '--uuid' or '--name'");
                Err(MyWayError::InvalidInput("You don't provided a identifier as uuid or name.".to_string()))
            }

        }



        // Verify "myway stacks"
        Commands::Stacks => {

            log.hey_mw("Searching all Stacks...");
            log.hey(">----------------------------------------------");

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

            let mut target_uuid = None;

            if let Some(uuid) = uuid {
                target_uuid = data.iter().find(|p| p.uuid == *uuid).map(|p| p.uuid.clone());
            }

            if let Some(name) = name {
                target_uuid = data.iter().find(|p| p.name == *name).map(|p| p.uuid.clone());
            }

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

                        if proj.versions.iter().any(|v| *v == *proj_version_new) {
                            log.hey("");
                            return Err(MyWayError::VersionAlreadyExists(format!("\"{}\" already exists a version.", proj_version_new)))
                        }

                        
                        log.hey_mw(&format!("{} -> {}", curr_ver, proj_version_new));
                        log.hey_mw(&format!("{} has a new version!", proj.name.bold()));
                        proj.versions.push(proj_version_new);
                        let _ = files.write(&data);

                        return Ok(());


                    } else {
                        return Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()));
                    }

                } else {
                    log.hey_mw("Use '--uuid' or '--name'");
                    return Err(MyWayError::InvalidInput("You don't provided a identifier as uuid or name.".to_string()));
                }

            }

            if *list {

                if let Some(uuid) = target_uuid {

                    if let Some(proj) = data.iter().find(|p| p.uuid == uuid) {

                        let project_name = proj.name.clone();
                        let project_versions = proj.versions.clone();

                        log.hey_mw(&format!("On \"{}\"'s versions", project_name));
                        log.hey_mw(&format!("{} version(s) found", project_versions.len()));
                        log.hey(">-------------------------------------------------------------------");
                        log.hey("");

                        for version in project_versions.iter() {

                            println!("- {}", version);

                        }   

                    } else {
                        return Err(MyWayError::ProjectNotFound("Project not exist or not found.".to_string()));
                    }

                } else {
                    log.hey_mw("Use '--uuid' or '--name'");
                    return Err(MyWayError::InvalidInput("You don't provided a identifier as uuid or name.".to_string()));
                }

            }

            Ok(())

        }




        Commands::Graveyard { uuid, name, list, kill, exject } => {

            log.hey_mw(&format!("{}", "On Graveyard!".red().bold()));
            log.hey_mw(&format!("{}", "Any change here will never be returned".red()));
            log.hey("");
            
            let mut target_uuid = None;

            if let Some(uuid) = uuid {
                target_uuid = data.iter().find(|p| p.uuid == *uuid).map(|p| p.uuid.clone());
            }

            if let Some(name) = name {
                target_uuid = data.iter().find(|p| p.name == *name).map(|p| p.uuid.clone());
            }

            if *kill {

                if let Some(uuid) = target_uuid {

                    if let Some(proj) = data.iter_mut().find(|p| p.uuid == *uuid) {

                        let proj = proj.clone();

                        log.hey_mw(&format!("You are about to kill \"{}\"", proj.name));
                        
                        let want = log.quest_option("KILL? [Y/n]", vec!["Y", "n"], "n").trim().to_string();

                        if want.is_empty() {
                            return Err(MyWayError::InvalidInput("Input is Empty".to_string()))
                        } 

                        if want == "Y" {

                            log.hey_mw(&format!("\"{}\" Killed!", format!("{}", proj.name)).red());
                            data.retain(|p| p.uuid != &*proj.uuid);
                            graveyard.push(proj);
                            let _ = files.write_graveyard(&graveyard);

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

                log.hey_mw(&format!("{} project(s) found", graveyard.len()));
                log.hey(">-------------------------------------------------------------------");
                log.hey("");
                
                for proj in graveyard.iter() {
                    println!("- {} {}", proj.name, proj.versions.last().expect(""))
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
                    let _ = files.write_graveyard(&graveyard);


                } else {
                    return Ok(())
                }

            }

            Ok(())

        }




        Commands::Status { uuid, name } => {

            log.hey_mw(&format!("{}", "On Way!"));
            log.hey("");

            let mut target_uuid = None;

            if let Some(uuid) = uuid {
                target_uuid = data.iter().find(|p| p.uuid == *uuid).map(|p| p.uuid.clone());
            }

            if let Some(name) = name {
                target_uuid = data.iter().find(|p| p.name == *name).map(|p| p.uuid.clone());
            }


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
                            is_finish: true,
                            time_created: proj.time_created.clone()
                        };


                        if let Some(proj) = data.iter_mut().find(|p| p.uuid == my_project.uuid) {
                            *proj = my_project;
                        }

                        let _ = files.write(&data);

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






        _ => {
            Ok(())
        }

    }

}
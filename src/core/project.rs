


#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Project {
    
    pub uuid: String, // project id
    pub name: String, // project name
    pub description: String, // project short desc
    
    pub stack: StackList, // project stacks list 
    pub mission: String, // project mission
    pub versions: VersionList,
    pub status: String, // project status (new, finished, fallen)

    pub time_created: String, // project time created
    pub your_think: String, // what the creator think about this project

    pub is_finish: bool,


    #[serde(default)]
    pub trace: Vec<String>,


}



#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {

    #[serde(default)]
    pub projects_created: u64,
    
    #[serde(default)]
    pub projects_finish: u64,

    #[serde(default)]
    pub projects_killed: u64,
    
    #[serde(default)]
    pub projects_revived: u64,

    #[serde(default)]
    pub projects_giveup: u64,

}



pub type GenericList = Vec<Project>;
pub type StackList = Vec<String>;
pub type VersionList = Vec<String>;




pub fn view_mission(mission: &str) -> String {
    match mission {
        "Y" => "YOU WILL FINISH THIS!".to_string(),
        "idk" => "Make a right decision, your TIME is more valious than gold.".to_string(),
        "n" => "FORGET THIS!".to_string(),
        _ => {
            "".to_string()
        }
    }
}
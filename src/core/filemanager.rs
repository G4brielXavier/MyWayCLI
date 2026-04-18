use std::fs;
use std::env;
use std::path::PathBuf;
use std::io::{Error, ErrorKind};

use dirs::data_local_dir;
use tequel::encrypt::{ TequelEncryption, TequelEncrypt };

use crate::core::project::{ GenericList, User };
use crate::core::errors::MyWayError;


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReturnReadType {
    GenericList(GenericList),
    User(User)
}

pub struct Fiman {
    pub teq_encrypt: TequelEncrypt,
    pub user_private_key: String,

    pub doc_path: PathBuf,
    
    pub mw_path: PathBuf,
    pub graveyard_path: PathBuf,
    pub user_path: PathBuf
}

impl Fiman {

    pub fn new() -> Result<Self, MyWayError> {
        
        let mut doc_path = data_local_dir().ok_or_else(|| {
            MyWayError::IoError(Error::new(
                ErrorKind::Other,
                "Could not find the system document directory"
            ))
        })?;

        doc_path.push("mywaycli");
        
        let mw_path = doc_path.join("myway_projects.tql");
        let graveyard_path = doc_path.join("graveyard_projects.tql");
        let user_path = doc_path.join("userdata.tql");

        let mut fiman = Self { 
            doc_path, 
            
            mw_path, 
            graveyard_path, 
            user_path,

            teq_encrypt: TequelEncrypt::new(), 
            user_private_key: "".to_string()
        };

        fiman.user_private_key = fiman.get_machine_seed();

        Ok(fiman)
    
    }


    fn helper_setup_file_encrypt(&mut self, content: &[u8], path: &PathBuf) -> Result<(), MyWayError> {
        
        let encrypted = self.teq_encrypt
            .encrypt(content, &self.user_private_key)
            .map_err(|e| MyWayError::IoError(Error::new(ErrorKind::Other, format!("{}", e))))?;

        let json_data = serde_json::to_string(&encrypted).map_err(|e| {
            MyWayError::IoError(e.into())
        })?;

        fs::write(&path, json_data).map_err(MyWayError::IoError)?;

        Ok(())

    }


    pub fn setup(&mut self) -> Result<(), MyWayError> {
            
        fs::create_dir_all(&self.doc_path).map_err(|e| { MyWayError::IoError(e) })?;
            
        // Migrate older users to new structure
        let mut old_dir = data_local_dir().unwrap();
        old_dir.push("MyWayCli");

        let old_path_projects = old_dir.join("myway_projects.json");
        let old_path_graveyard = old_dir.join("graveyard_projects.json");

        if old_path_projects.exists() {
            println!("Migrating your projects to a new secure vault...");

            let content = fs::read(&old_path_projects).map_err(MyWayError::IoError)?;
            self.helper_setup_file_encrypt(&content, &self.mw_path.clone())?;
            fs::remove_file(old_path_projects).ok();
        }

        if old_path_graveyard.exists() {
            println!("Migrating your graveyard to a new secure vault...");

            let content = fs::read(&old_path_graveyard).map_err(MyWayError::IoError)?;
            self.helper_setup_file_encrypt(&content, &self.graveyard_path.clone())?;
            fs::remove_file(old_path_graveyard).ok();
        }


        // Create data files if already not exist

        if !self.mw_path.exists() {
            self.helper_setup_file_encrypt("[]".as_bytes(), &self.mw_path.clone())?;
        }

        if !self.user_path.exists() {
            let user = User { projects_created: 0, projects_finish: 0, projects_killed: 0, projects_revived: 0, projects_giveup: 0 };
            let user_data = serde_json::to_string_pretty(&user).map_err(|e| MyWayError::IoError(e.into()))?;

            self.helper_setup_file_encrypt(user_data.as_bytes(), &self.user_path.clone())?;
        }

        if !self.graveyard_path.exists() {
            self.helper_setup_file_encrypt("[]".as_bytes(), &self.graveyard_path.clone())?;
        }

        Ok(())

    }

    pub fn write(&mut self, data: &ReturnReadType, path: &PathBuf) -> Result<(), MyWayError> {

        let json_data = serde_json::to_vec(data).map_err(|e| MyWayError::IoError(e.into()))?;
    
        let mut tmp_path = path.clone();
        tmp_path.set_extension("tmp");

        fs::write(&tmp_path, &json_data).map_err(MyWayError::IoError)?;

        fs::rename(&tmp_path, path).map_err(|e| {
            let _ = std::fs::remove_file(&tmp_path);
            MyWayError::IoError(e)
        })?;

        self.helper_setup_file_encrypt(&json_data, path)?;

        Ok(())
    }

    pub fn read(&mut self, path: &PathBuf) -> Result<ReturnReadType, MyWayError> {

        let content = fs::read_to_string(&path).map_err(MyWayError::IoError)?;
        let enc_struct: TequelEncryption = serde_json::from_str(&content).map_err(|e| MyWayError::IoError(e.into()))?;

        let decrypted = self.teq_encrypt.decrypt(&enc_struct, &self.user_private_key).map_err(|e| {
            MyWayError::IoError(Error::new(ErrorKind::Other, format!("TEQUEL FAILED: {}", e)))
        })?;

        let result = serde_json::from_str::<ReturnReadType>(&decrypted).map_err(|e| {
            MyWayError::IoError(Error::new(ErrorKind::Other, format!("SERDE FAILED: {}", e)))
        })?;

        Ok(result)

    }









    pub fn get_machine_seed(&self) -> String {

        let user = env::var("USERNAME")
            .or_else(|_| env::var("USER"))
            .unwrap_or_else(|_| "unknown_user".to_string());

        let computer = env::var("COMPUTERNAME")
            .or_else(|_| env::var("HOSTNAME"))
            .unwrap_or_else(|_| "unknown_machine".to_string());

        format!("{}-{}", user, computer).trim().to_lowercase()
    }

}
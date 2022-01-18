use serde::{Serialize, Deserialize};
use crate::utils::schema::folders;
use bigdecimal::{BigDecimal, ToPrimitive, FromPrimitive};
use schemars::JsonSchema;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Folder{
    pub id: i32,
    pub folder_num: String,
    pub id_customer: i32,
    pub id_housing: i32,
    pub tax_income: BigDecimal,
    pub situation: String,

}

#[derive(Insertable, Serialize, Deserialize, Clone)]
#[table_name = "folders"]
pub struct NewFolder {
    pub folder_num: String,
    pub id_customer : i32,
    pub id_housing : i32,
    pub tax_income: BigDecimal,
    pub situation: String,
}

impl NewFolder{
    pub fn new(send_new_folder : &SendableNewFolder) -> NewFolder{
        NewFolder{
            folder_num: send_new_folder.folder_num.clone(),
            id_customer: send_new_folder.id_customer,
            id_housing: send_new_folder.id_housing,
            tax_income: BigDecimal::from_f32(send_new_folder.tax_income).unwrap(),
            situation: send_new_folder.situation.clone(),
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, JsonSchema)]
pub struct SendableFolder{
    pub id: i32,
    pub folder_num: String,
    pub id_customer: i32,
    pub id_housing: i32,
    pub tax_income: f32,
    pub situation: String,

}

impl SendableFolder {
    
    pub fn new(folder : Folder) -> SendableFolder{
        SendableFolder{ 
            id: folder.id, 
            folder_num: folder.folder_num, 
            id_customer: folder.id_customer, 
            id_housing: folder.id_housing, 
            tax_income: folder.tax_income.to_f32().unwrap(),
            situation: folder.situation
        }
    }

    pub fn convert_vec(folders : Vec<Folder>) -> Vec<SendableFolder>{
        let mut sendable_folders = Vec::new();
        for folder in folders{
            sendable_folders.push(SendableFolder::new(folder));
        }
        return sendable_folders;
    }
}

#[derive(Queryable, Serialize, Deserialize, JsonSchema)]
pub struct SendableNewFolder {
    pub folder_num: String,
    pub id_customer : i32,
    pub id_housing : i32,
    pub tax_income: f32,
    pub situation: String,
}

impl SendableNewFolder {
    
    pub fn new(folder : NewFolder) -> SendableNewFolder{
        SendableNewFolder{ 
            folder_num: folder.folder_num, 
            id_customer: folder.id_customer, 
            id_housing: folder.id_housing, 
            tax_income: folder.tax_income.to_f32().unwrap(),
            situation: folder.situation
        }
    }

    pub fn convert_vec(folders : Vec<NewFolder>) -> Vec<SendableNewFolder>{
        let mut sendable_folders = Vec::new();
        for folder in folders{
            sendable_folders.push(SendableNewFolder::new(folder));
        }
        return sendable_folders;
    }
}
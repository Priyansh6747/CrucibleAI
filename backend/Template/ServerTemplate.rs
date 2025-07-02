use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer , Responder,HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HTTPClient;
use async_trait::async_trait;

use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use log::debug;

#[derive(Serialize,Deserialize,Debug,Clone, PartialEq,Eq)]
struct Task {
    id:u64,
    name:String,
    status:bool,
}
#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq)]
struct User {
    id:u64,
    name:String,
    password:String,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
struct DB {
    users:HashMap<u64,User>,
    tasks:HashMap<u64,Task>,
}

impl DB {
    fn new() -> Self {
        Self{
            users:HashMap::new(),
            tasks:HashMap::new(),
        }
    }

    //CRUD operations
    fn insert(&mut self , task : Task) {
        self.tasks.insert(task.id,task);
    }
    fn get_task(&self ,id : u64) -> Option<&Task> {
        self.tasks.get(&id)
    }
    fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }
    fn delete_task(&mut self , id : u64) {
        self.tasks.remove(&id);
    }
    fn update_task(&mut self , id : u64 , task : Task) {
        self.tasks.insert(id,task);
    }
    
    
    fn insert_user(&mut self , user : User) {
        self.users.insert(user.id,user);
    }
    fn get_user(&self ,id : u64) -> Option<&User> {
        self.users.get(&id)
    }
    fn get_all_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }
    fn delete_user(&mut self , id : u64) {
        self.users.remove(&id);
    }
    fn update_user(&mut self , id : u64 , user : User) {
        self.users.insert(id,user);
    }
    fn get_user_by_name(&self , name : &str) -> Option<&User> {
        self.users.values().find(|user| user.name == name)
    }

    fn save_to_file(&self) -> Result<(),std::io::Error> {
        let data = serde_json::to_string(self)?;
        let mut file = fs::OpenOptions::new().write(true).create(true).open("db.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
    fn load_from_file() -> Result<Self,std::io::Error> {
        let file_content = fs::read_to_string("db.json")?;
        let db = serde_json::from_str(&file_content)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<DB>,
}

async fn create_task(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}
async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get_task(id.into_inner()) { 
        Some(task) => {HttpResponse::Ok().json(task)}
        None => {HttpResponse::NotFound().finish()}
    }
}

async fn get_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    HttpResponse::Ok().json(db.get_all_tasks())
}


#[test]
fn test_db() {
    let mut db = DB::new();
    db.insert(Task{id:1,name:"test1".to_string(),status:true});
    db.insert(Task{id:2,name:"test2".to_string(),status:true});
    db.insert(Task{id:3,name:"test3".to_string(),status:true});
    db.insert_user(User{id:1,name:"test1".to_string(),password:"testpassword".to_string()});
    db.insert_user(User{id:2,name:"test1".to_string(),password:"testpassword".to_string()});
    db.save_to_file().unwrap();
    let db2 = DB::load_from_file().unwrap();
    assert_eq!(db.tasks, db2.tasks);
    assert_eq!(db.users, db2.users);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = DB::load_from_file().unwrap_or_else(|_| DB::new());
    let data = web::Data::new(AppState{db: Mutex::new(db)});
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .route("/tasks", web::post().to(create_task))
            .route("/tasks/{id}", web::get().to(read_task))
            .route("/tasks", web::get().to(get_all_tasks))
    }).bind("127.0.0.1:8080")?.run().await
}
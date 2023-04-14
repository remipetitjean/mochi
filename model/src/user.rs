use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::id::generate_id;
use crate::error::Result;

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub handler: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize)]
pub struct UserForCreate {
    pub email: String,
    pub handler: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone)]
pub struct UserController {
    users_store: Arc<Mutex<Vec<Option<User>>>>,
}

impl UserController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            users_store: Arc::default(),
        })
    }
}

impl UserController {
    pub async fn create(&self, user_for_create: UserForCreate) -> Result<User> {
        let mut store = self.users_store.lock().unwrap();
        let id = generate_id();
        let user = User {
            id,
            email: user_for_create.email,
            handler: user_for_create.handler,
            first_name: user_for_create.first_name,
            last_name: user_for_create.last_name,
        };
        store.push(Some(user.clone()));
        Ok(user)
    }
}

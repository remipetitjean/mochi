use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::id::generate_id;

#[derive(Debug, Clone)]
pub struct UserCreateError;
impl std::fmt::Display for UserCreateError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "cannot create user {self:}")
    }
}
impl std::error::Error for UserCreateError {}

#[derive(Debug, Clone)]
pub struct UserReadError {
    id: String,
}
impl std::fmt::Display for UserReadError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "cannot read user {}", self.id)
    }
}
impl std::error::Error for UserReadError {}

#[derive(Debug, Clone)]
pub struct UserListError;
impl std::fmt::Display for UserListError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "cannot list users")
    }
}
impl std::error::Error for UserListError {}

#[derive(Debug, Clone)]
pub struct UserDeleteError {
    id: String,
}
impl std::fmt::Display for UserDeleteError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "cannot delete user {self:}")
    }
}
impl std::error::Error for UserDeleteError {}

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
    pub async fn new() -> Self {
        Self {
            users_store: Arc::default(),
        }
    }
}

impl UserController {
    pub async fn create(&self, user_for_create: UserForCreate) -> Result<User, UserCreateError> {
        let mut store = self.users_store.lock().unwrap();
        let id = generate_id();

        if user_for_create.first_name == "oups" {
            return Err(UserCreateError {});
        }

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

    pub async fn read(&self, id: String) -> Result<User, UserReadError> {
        let mut store = self.users_store.lock().unwrap();
        let user = store.get_mut(0).and_then(|b| b.take());
        user.ok_or(UserReadError { id })
    }

    pub async fn list(&self) -> Result<Vec<User>, UserListError> {
        Err(UserListError)
    }

    pub async fn delete(&self, id: String) -> Result<User, UserDeleteError> {
        Err(UserDeleteError { id })
    }
}

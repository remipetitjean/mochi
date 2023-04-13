use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::ctx::Ctx;
use crate::{Error, Result};

#[derive(Clone, Debug, Serialize)]
pub struct Bot {
    pub id: u64,
    pub cid: u64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct BotForCreate {
    pub name: String,
}

#[derive(Clone)]
pub struct ModelController {
    bots_store: Arc<Mutex<Vec<Option<Bot>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            bots_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create(&self, ctx: Ctx, bot_for_create: BotForCreate) -> Result<Bot> {
        let mut store = self.bots_store.lock().unwrap();
        let id = store.len() as u64;
        let bot = Bot {
            id,
            cid: ctx.user_id(),
            name: bot_for_create.name,
        };
        store.push(Some(bot.clone()));
        Ok(bot)
    }

    pub async fn list(&self, _ctx: Option<Ctx>) -> Result<Vec<Bot>> {
        let store = self.bots_store.lock().unwrap();
        let bots = store.iter().filter_map(|b| b.clone()).collect();
        Ok(bots)
    }

    pub async fn delete(&self, _ctx: Ctx, id: u64) -> Result<Bot> {
        let mut store = self.bots_store.lock().unwrap();
        let bot = store.get_mut(id as usize).and_then(|b| b.take());
        bot.ok_or(Error::BotDeleteFailIdNotFound { id })
    }
}

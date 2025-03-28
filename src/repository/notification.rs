use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::model::notification::Notification;

// Singleton of Database
lazy_static! {
    static ref NOTIFICATION: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;

impl NotificationRepository {}

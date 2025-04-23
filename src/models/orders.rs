use serde::{Serialize, Deserialize};
use uuid::Uuid; // Geração de IDs unicos e manipulacao de tempo
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid, 
    pub client_id: Uuid,
    pub restaurant_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Conta {
    pub id: String,
    pub nome: String,
    pub saldo: f64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transacao {
    pub id: String,
    pub conta_origem: String,
    pub conta_destino: String,
    pub valor: f64,
    pub tipo: String, 
    pub data: String,
}

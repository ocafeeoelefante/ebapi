use sqlx::sqlite::SqlitePool;

pub async fn init_db(pool: &SqlitePool) {

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS contas (
            id TEXT PRIMARY KEY,
            nome TEXT NOT NULL,
            saldo REAL NOT NULL
        );
        CREATE TABLE IF NOT EXISTS transacoes (
            id TEXT PRIMARY KEY,
            conta_origem TEXT,
            conta_destino TEXT,
            valor REAL NOT NULL,
            tipo TEXT NOT NULL,
            data TEXT NOT NULL,
            FOREIGN KEY(conta_origem) REFERENCES contas(id),
            FOREIGN KEY(conta_destino) REFERENCES contas(id)
        );
        "#
    )
    .execute(pool)
    .await
    .expect("Erro ao criar tabelas");
}

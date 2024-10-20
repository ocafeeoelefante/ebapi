use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{Conta, Transacao};

pub async fn criar_conta(
    pool: web::Data<SqlitePool>,
    conta: web::Json<Conta>,
) -> HttpResponse {
    let nova_conta = Conta {
        id: Uuid::new_v4().to_string(),
        nome: conta.nome.clone(),
        saldo: conta.saldo,
    };

    let result = sqlx::query!(
        "INSERT INTO contas (id, nome, saldo) VALUES (?, ?, ?)",
        nova_conta.id,
        nova_conta.nome,
        nova_conta.saldo
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(&nova_conta),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erro: {}", e)),
    }
}

pub async fn obter_conta(
    pool: web::Data<SqlitePool>,
    id: web::Path<String>,
) -> HttpResponse {
    let result = sqlx::query_as!(
        Conta,
        "SELECT id, nome, saldo FROM contas WHERE id = ?",
        id.into_inner()
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(conta) => HttpResponse::Ok().json(conta),
        Err(_) => HttpResponse::NotFound().body("Conta não encontrada"),
    }
}

pub async fn transferencia_pix(
    pool: web::Data<SqlitePool>,
    transacao: web::Json<Transacao>,
) -> HttpResponse {
    let mut tx = pool.begin().await.unwrap();

    let origem = &transacao.conta_origem;
    let destino = &transacao.conta_destino;
    let valor = transacao.valor;

    let saldo_origem: (f64,) = sqlx::query_as("SELECT saldo FROM contas WHERE id = ?")
        .bind(origem)
        .fetch_one(&mut tx)
        .await
        .unwrap_or((0.0,));

    if saldo_origem.0 < valor {
        return HttpResponse::BadRequest().body("Saldo insuficiente");
    }

    sqlx::query("UPDATE contas SET saldo = saldo - ? WHERE id = ?")
        .bind(valor)
        .bind(origem)
        .execute(&mut tx)
        .await
        .unwrap();

    sqlx::query("UPDATE contas SET saldo = saldo + ? WHERE id = ?")
        .bind(valor)
        .bind(destino)
        .execute(&mut tx)
        .await
        .unwrap();

    let nova_transacao = Transacao {
        id: Uuid::new_v4().to_string(),
        conta_origem: origem.clone(),
        conta_destino: destino.clone(),
        valor,
        tipo: "PIX".to_string(),
        data: chrono::Local::now().to_string(),
    };

    sqlx::query!(
        "INSERT INTO transacoes (id, conta_origem, conta_destino, valor, tipo, data) VALUES (?, ?, ?, ?, ?, ?)",
        nova_transacao.id,
        nova_transacao.conta_origem,
        nova_transacao.conta_destino,
        nova_transacao.valor,
        nova_transacao.tipo,
        nova_transacao.data
    )
    .execute(&mut tx)
    .await
    .unwrap();

    tx.commit().await.unwrap();

    HttpResponse::Ok().json("Transferência realizada com sucesso")
}

pub async fn obter_saldo(
    pool: web::Data<SqlitePool>,
    id: web::Path<String>,
) -> HttpResponse {
    let result: Result<(f64,), sqlx::Error> = sqlx::query_as("SELECT saldo FROM contas WHERE id = ?")
        .bind(id.into_inner())
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok((saldo,)) => HttpResponse::Ok().json(saldo),
        Err(_) => HttpResponse::NotFound().body("Conta não encontrada"),
    }
}

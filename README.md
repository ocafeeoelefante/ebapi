# EthBank API

EthBank é uma API desenvolvida em **Rust** para gerenciar transações financeiras, incluindo funcionalidades como transferências via **PIX**, pagamentos com **crédito** e **débito**, e gestão de contas. Esta API foi projetada para ser eficiente e segura, utilizando o **Rust** como base para garantir alta performance e segurança.

## Funcionalidades

- **Criação de contas**: Crie novas contas com saldo inicial.
- **Transações PIX**: Realize transferências instantâneas entre contas utilizando identificadores como chave aleatória ou CPF.
- **Pagamentos com Crédito/Débito**: Simule pagamentos e ajuste o saldo de contas conforme o tipo de transação.
- **Consulta de Saldo**: Consulte o saldo e o histórico de transações de uma conta específica.

## Tecnologias

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Actix-Web](https://img.shields.io/badge/Actix_Web-2B6CB3?style=for-the-badge)
![SQLite](https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white)
![Serde](https://img.shields.io/badge/Serde-50C878?style=for-the-badge)
![UUID](https://img.shields.io/badge/UUID-4B0082?style=for-the-badge)
![Tokio](https://img.shields.io/badge/Tokio-008080?style=for-the-badge)


## Endpoints

### 1. Criar Conta

- **Método**: `POST`
- **Endpoint**: `/api/contas`
- **Descrição**: Cria uma nova conta bancária.
- **Exemplo de Requisição**:

```json
{
  "nome": "Liam G.",
  "saldo": 1000.00
}

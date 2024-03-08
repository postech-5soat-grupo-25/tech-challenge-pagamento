use std::sync::Arc;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket::State;
use rocket::Data;
use rocket_okapi::{openapi, openapi_get_routes};
use tokio::sync::Mutex;

use crate::api::error_handling::ErrorResponse;
use crate::api::request_guards::authentication_guard::AuthenticatedUser;
use crate::controllers::pedido_controller::PedidoController;
use crate::entities::pagamento::Pagamento;
use crate::entities::pedido::Pedido;

use crate::traits::{
    cliente_gateway::ClienteGateway, pagamento_gateway::PagamentoGateway,
    pedido_gateway::PedidoGateway, produto_gateway::ProdutoGateway,
};
use crate::use_cases::pedidos_e_pagamentos_use_case::CreatePedidoInput;

#[openapi(tag = "Pedidos")]
#[get("/")]
async fn get_pedidos(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    _logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pedidos = pedido_controller.get_pedidos().await?;
    Ok(Json(pedidos))
}

#[openapi(tag = "Pedidos")]
#[get("/<id>")]
async fn get_pedido_by_id(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    id: usize,
    __logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pedido = pedido_controller.get_pedido_by_id(id).await?;
    Ok(Json(pedido))
}

#[openapi(tag = "Pedidos")]
#[post("/", data = "<pedido_input>")]
async fn post_novo_pedido(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    pedido_input: Json<CreatePedidoInput>,
) -> Result<Json<Pedido>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pedido_input = pedido_input.into_inner();
    let novo_pedido = pedido_controller.novo_pedido(pedido_input).await?;
    Ok(Json(novo_pedido))
}

#[openapi(tag = "Pedidos")]
#[get("/novos")]
async fn get_pedidos_novos(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    __logged_user_info: AuthenticatedUser,
) -> Result<Json<Vec<Pedido>>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pedidos_novos = pedido_controller.get_pedidos_novos().await?;
    Ok(Json(pedidos_novos))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/status/<status>")]
async fn put_status_pedido(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    id: usize,
    status: &str,
    __logged_user_info: AuthenticatedUser,
) -> Result<Json<Pedido>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pedido = pedido_controller.atualiza_status_pedido(id, status).await?;
    Ok(Json(pedido))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/cliente/<cliente_id>")]
async fn put_cliente_pedido(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    id: usize,
    cliente_id: usize,
) -> Result<Json<Pedido>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pedido = pedido_controller
        .atualiza_cliente_pedido(id, cliente_id)
        .await?;
    Ok(Json(pedido))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/produto/<categoria>/<produto_id>")]
async fn put_produto_by_categoria(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    id: usize,
    categoria: &str,
    produto_id: usize,
) -> Result<Json<Pedido>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pedido = pedido_controller
        .atualiza_produto_by_categoria(id, categoria, produto_id)
        .await?;

    Ok(Json(pedido))
}

#[openapi(tag = "Pedidos")]
#[get("/<id>/pagamento")]
async fn get_pagamento_by_pedido_id(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    id: usize,
) -> Result<Json<Pagamento>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pagamento = pedido_controller.get_pagamento_by_pedido_id(id).await?;

    Ok(Json(pagamento))
}

#[openapi(tag = "Pedidos")]
#[post("/<id>/pagamento")]
async fn pagar(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    id: usize,
) -> Result<Json<Pagamento>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );
    let pagamento = pedido_controller.pagar(id).await?;

    Ok(Json(pagamento))
}

#[openapi(tag = "Pedidos")]
#[put("/<id>/pagamento", data = "<data>")]
async fn webhook_pagamento(
    pedido_repository: &State<Arc<Mutex<dyn PedidoGateway + Sync + Send>>>,
    cliente_repository: &State<Arc<Mutex<dyn ClienteGateway + Sync + Send>>>,
    produto_repository: &State<Arc<Mutex<dyn ProdutoGateway + Sync + Send>>>,
    pagamento_repository: &State<Arc<Mutex<dyn PagamentoGateway + Sync + Send>>>,
    id: usize,
    data: Json<Value>,
) -> Result<Json<Pagamento>, Status> {
    let pedido_controller = PedidoController::new(
        pedido_repository.inner().clone(),
        cliente_repository.inner().clone(),
        produto_repository.inner().clone(),
        pagamento_repository.inner().clone(),
    );

    let data_pagamento = data.into_inner();
    let pagamento = pedido_controller.webhook_pagamento(id, data_pagamento).await?;
    Ok(Json(pagamento))
}

pub fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![
        get_pedidos,
        post_novo_pedido,
        get_pedidos_novos,
        put_status_pedido,
        put_cliente_pedido,
        put_produto_by_categoria,
        get_pagamento_by_pedido_id,
        pagar,
        webhook_pagamento,
    ]
}

#[catch(404)]
fn pedido_not_found() -> Json<ErrorResponse> {
    let error = ErrorResponse {
        msg: "Pedido não encontrado!".to_string(),
        status: 404,
    };
    Json(error)
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![pedido_not_found]
}

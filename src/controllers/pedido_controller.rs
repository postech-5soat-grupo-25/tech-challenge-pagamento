use std::sync::Arc;

use rocket::serde::json::Json;
use tokio::sync::Mutex;

use crate::base::domain_error::DomainError;
use crate::entities::pedido::{self, Pedido};

use crate::traits::{
    pedido_repository::PedidoRepository,
    cliente_repository::ClienteRepository,
    produto_repository::ProdutoRepository,
    pagamento_port::PagamentoPort,
};

use crate::use_cases::{
    pedidos_e_pagamentos_use_case::PedidosEPagamentosUseCase,
    preparacao_e_entrega_use_case::PreparacaoeEntregaUseCase,
    pedidos_e_pagamentos_use_case::CreatePedidoInput,
};

pub struct PedidoController {
    pedidos_e_pagamentos_use_case: PedidosEPagamentosUseCase,
    preparacao_e_entrega_use_case: PreparacaoeEntregaUseCase,
}

impl PedidoController {
    pub fn new(
        pedido_repository: Arc<Mutex<dyn PedidoRepository + Sync + Send>>,
        cliente_repository: Arc<Mutex<dyn ClienteRepository + Sync + Send>>,
        produto_repository: Arc<Mutex<dyn ProdutoRepository + Sync + Send>>,
        pagamento_adapter: Arc<Mutex<dyn PagamentoPort + Sync + Send>>,
    ) -> PedidoController {
        let pedidos_e_pagamentos_use_case = PedidosEPagamentosUseCase::new(
            pedido_repository.clone(),
            cliente_repository,
            produto_repository,
            pagamento_adapter,
        );
        let preparacao_e_entrega_use_case = PreparacaoeEntregaUseCase::new(pedido_repository);

        PedidoController {
            pedidos_e_pagamentos_use_case,
            preparacao_e_entrega_use_case,
        }
    }

    pub async fn get_pedidos(
        &self
    ) -> Result<Vec<Pedido>, DomainError> {
        self.pedidos_e_pagamentos_use_case.lista_pedidos().await
    }

    pub async fn get_pedido_by_id(
        &self,
        id: usize,
    ) -> Result<Pedido, DomainError> {
        self.pedidos_e_pagamentos_use_case
            .seleciona_pedido_por_id(id)
            .await
    }

    pub async fn novo_pedido(
        &self,
        pedido_input: CreatePedidoInput,
    ) -> Result<Pedido, DomainError> {
        self.pedidos_e_pagamentos_use_case
            .novo_pedido(pedido_input)
            .await
    }

    pub async fn get_pedidos_novos(
        &self,
    ) -> Result<Vec<Pedido>, DomainError> {
        self.preparacao_e_entrega_use_case.get_pedidos_novos().await
    }

    pub async fn atualiza_status_pedido(
        &self,
        id: usize,
        status: &str,
    ) -> Result<Pedido, DomainError> {
        let status = match status {
            "Cancelado" => pedido::Status::Cancelado,
            "EmPreparacao" => pedido::Status::EmPreparacao,
            "Finalizado" => pedido::Status::Finalizado,
            "Invalido" => pedido::Status::Invalido,
            "Pendente" => pedido::Status::Pendente,
            "Pronto" => pedido::Status::Pronto,
            "Recebido" => pedido::Status::Recebido,
            _ => return Err(DomainError::Invalid("Status inválido".to_string())),
        };
        self.preparacao_e_entrega_use_case
            .atualiza_status(id, status)
            .await
    }

    pub async fn atualiza_cliente_pedido(
        &self,
        id: usize,
        cliente_id: usize,
    ) -> Result<Pedido, DomainError> {
        self.pedidos_e_pagamentos_use_case
            .adicionar_cliente(id, cliente_id)
            .await
    }

    pub async fn atualiza_produto_by_categoria(
        &self,
        id: usize,
        categoria: &str,
        produto_id: usize,
    ) -> Result<Pedido, DomainError> {
        match categoria {
            "Lanche" => {
                self.pedidos_e_pagamentos_use_case
                    .adicionar_lanche_com_personalizacao(id, produto_id)
                    .await
            }
            "Acompanhamento" => {
                self.pedidos_e_pagamentos_use_case
                    .adicionar_acompanhamento(id, produto_id)
                    .await
            }
            "Bebida" => {
                self.pedidos_e_pagamentos_use_case
                    .adicionar_bebida(id, produto_id)
                    .await
            }
            _ => Err(DomainError::Invalid("Categoria inválida".to_string()))
        }
    }

    pub async fn pagar(
        &self,
        id: usize,
    ) -> Result<Pedido, DomainError> {
        self.pedidos_e_pagamentos_use_case
            .realizar_pagamento_do_pedido(id)
            .await
    }
}
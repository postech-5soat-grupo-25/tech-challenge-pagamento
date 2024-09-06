package com.tech.challenge.pagamentos.dto;

public class PaymentMessageDto {
    private String pedidoId;
    private String pagamentoId;
    private String statusPagamento;


    public PaymentMessageDto(String pedidoId, String pagamentoId, String statusPagamento) {
        this.pedidoId = pedidoId;
        this.pagamentoId = pagamentoId;
        this.statusPagamento = statusPagamento;
    }

    public String getPedidoId() {
        return pedidoId;
    }
    public void setPedidoId(String pedidoId) {
        this.pedidoId = pedidoId;
    }
    public String getPagamentoId() {
        return pagamentoId;
    }
    public void setPagamentoId(String pagamentoId) {
        this.pagamentoId = pagamentoId;
    }
    public String getStatusPagamento() {
        return statusPagamento;
    }
    public void setStatusPagamento(String statusPagamento) {
        this.statusPagamento = statusPagamento;
    }


}

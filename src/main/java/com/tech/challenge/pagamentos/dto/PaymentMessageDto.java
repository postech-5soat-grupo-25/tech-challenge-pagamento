package com.tech.challenge.pagamentos.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import com.fasterxml.jackson.databind.annotation.JsonNaming;

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy.class)
public class PaymentMessageDto {
    private Integer pedidoId;
    private String pagamentoId;
    private String status;

    public PaymentMessageDto(Integer pedidoId, String pagamentoId, String status) {
        this.pedidoId = pedidoId;
        this.pagamentoId = pagamentoId;
        this.status = status;
    }

    @JsonProperty("pedido_id")
    public Integer getPedidoId() {
        return pedidoId;
    }

    public void setPedidoId(Integer pedidoId) {
        this.pedidoId = pedidoId;
    }

    @JsonProperty("pagamento_id")
    public String getPagamentoId() {
        return pagamentoId;
    }

    public void setPagamentoId(String pagamentoId) {
        this.pagamentoId = pagamentoId;
    }

    @JsonProperty("status")
    public String getStatusPagamento() {
        return status;
    }

    public void setStatusPagamento(String status) {
        this.status = status;
    }

    @Override
    public String toString() {
        return "PaymentMessageDto{" +
                "pedido_id='" + pedidoId + '\'' +
                ", pagamento_id='" + pagamentoId + '\'' +
                ", status='" + status + '\'' +
                '}';
    }
}

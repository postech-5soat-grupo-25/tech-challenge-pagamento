package com.tech.challenge.pagamentos.dto;

import com.fasterxml.jackson.annotation.JsonProperty;

public class MockPagamentoRequest {

    @JsonProperty("webhook_url")
    private String webhookUrl;

    private Double value;

    public MockPagamentoRequest(String webhookUrl, Double value) {
        this.webhookUrl = webhookUrl;
        this.value = value;
    }

    public String getWebhookUrl() {
        return webhookUrl;
    }

    public void setWebhookUrl(String webhookUrl) {
        this.webhookUrl = webhookUrl;
    }

    public Double getValue() {
        return value;
    }

    public void setValue(Double value) {
        this.value = value;
    }
}

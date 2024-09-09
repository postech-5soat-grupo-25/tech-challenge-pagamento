package com.tech.challenge.pagamentos.dto;

public class WebhookPagamentoDto {
    private String payment_status;
    private String payment_code;
    
    public WebhookPagamentoDto(String payment_status, String payment_code) {
        this.payment_status = payment_status;
        this.payment_code = payment_code;
    }

    public String getPaymentStatus() {
        return payment_status;
    }

    public void setPaymentStatus(String payment_status) {
        this.payment_status = payment_status;
    }

    public String getPaymentCode() {
        return payment_code;
    }

    public void setPaymentCode(String payment_code) {
        this.payment_code = payment_code;
    }
}

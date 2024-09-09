package com.tech.challenge.pagamentos.controller;

import org.springframework.amqp.rabbit.core.RabbitTemplate;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.server.ResponseStatusException;

import com.tech.challenge.pagamentos.config.RabbitMQConfig;
import com.tech.challenge.pagamentos.dto.PaymentMessageDto;
import com.tech.challenge.pagamentos.dto.WebhookPagamentoDto;
import com.tech.challenge.pagamentos.models.Pagamento;
import com.tech.challenge.pagamentos.repository.PagamentosRepository;

@RestController
@RequestMapping("/webhook")
public class PagamentosWebhookController {

    @Autowired
    private PagamentosRepository pagamentoRepository;

    @Autowired
    private RabbitTemplate rabbitTemplate;

    @PostMapping
    public void webhookPagamento(
            @RequestBody WebhookPagamentoDto webhookPayload,
            @RequestParam(name = "payment_id", required = true) String paymentId) {

        Pagamento pagamentoSalvo = pagamentoRepository.findById(paymentId).orElseThrow(
                () -> new ResponseStatusException(HttpStatus.NOT_FOUND, "Paymento Not Found"));

        if ("success".equals(webhookPayload.getPaymentStatus())) {

            // Conversão de idPedido para Integer
            Integer idPedidoInteger = null;
            try {
                idPedidoInteger = Integer.parseInt(pagamentoSalvo.getIdPedido());
            } catch (NumberFormatException e) {
                throw new ResponseStatusException(HttpStatus.BAD_REQUEST, "Invalid order ID format");
            }

            PaymentMessageDto payload = new PaymentMessageDto(
                    idPedidoInteger, // Aqui está o idPedido convertido para Integer
                    pagamentoSalvo.getId(),
                    "Aprovado");

            pagamentoSalvo.setEstado("Aprovado");
            rabbitTemplate.convertAndSend(RabbitMQConfig.EXCHANGE_NAME, "pagamentos", payload);
        } else {
            pagamentoSalvo.setEstado("Recusado");
        }

        pagamentoRepository.save(pagamentoSalvo);
    }
}

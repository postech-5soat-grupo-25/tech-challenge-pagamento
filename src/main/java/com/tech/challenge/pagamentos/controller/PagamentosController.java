package com.tech.challenge.pagamentos.controller;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatusCode;
import org.springframework.http.ResponseEntity;
import org.springframework.web.client.RestTemplate;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.tech.challenge.pagamentos.dto.MockPagamentoRequest;
import com.tech.challenge.pagamentos.models.Pagamento;
import com.tech.challenge.pagamentos.repository.PagamentosRepository;

import java.time.LocalDateTime;
import java.util.List;

@RestController
@RequestMapping("/pagamentos")
public class PagamentosController {

    @Autowired
    private PagamentosRepository pagamentoRepository;

    @Autowired
    private ObjectMapper objectMapper;

    @Autowired
    private RestTemplate restTemplate;

    @PostMapping
    public Pagamento criarPagamento(@RequestBody Pagamento pagamento) {
        pagamento.setDataCriacao(LocalDateTime.now());

        Pagamento pagamentoSalvo = pagamentoRepository.save(pagamento);

        MockPagamentoRequest mockRequest = new MockPagamentoRequest("http://pagamentos-service:32100/webhook?payment_id=" + pagamento.getId(), pagamento.getValor());

        try {
            String jsonBody = objectMapper.writeValueAsString(mockRequest);
            System.out.println("Body da requisição POST: " + jsonBody);
        } catch (JsonProcessingException e) {
            e.printStackTrace();
        }

        ResponseEntity<String> response = restTemplate.postForEntity("http://mock-pagamentos-svc:9000/payment/", mockRequest, String.class);
        
        HttpStatusCode statusCode = response.getStatusCode();
        
        if (statusCode.is2xxSuccessful()) {
            System.out.println("Requisição POST foi bem-sucedida.");
        } else {
            System.out.println("Requisição POST falhou com status: " + statusCode);
            pagamento.setEstado("Falha");
            pagamentoSalvo = pagamentoRepository.save(pagamento);
        }

        return pagamentoSalvo;
    }

    @GetMapping
    public List<Pagamento> buscaPagamentos(){
        return pagamentoRepository.findAll();
    }


}
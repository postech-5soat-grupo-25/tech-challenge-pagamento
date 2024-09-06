package com.tech.challenge.pagamentos.repository;

import org.springframework.data.mongodb.repository.MongoRepository;

import com.tech.challenge.pagamentos.models.Pagamento;

public interface PagamentosRepository extends MongoRepository<Pagamento, String> {
}
package com.tech.challenge.pagamentos.config;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.client.BufferingClientHttpRequestFactory;
import org.springframework.http.client.ClientHttpRequestFactory;
import org.springframework.http.client.HttpComponentsClientHttpRequestFactory;
import org.springframework.web.client.RestTemplate;

@Configuration
public class AppConfig {

    @Bean
    public RestTemplate restTemplate() {
        // Cria uma fábrica de requisições HTTP básica
        HttpComponentsClientHttpRequestFactory basicRequestFactory = new HttpComponentsClientHttpRequestFactory();

        // Envolve a fábrica básica com BufferingClientHttpRequestFactory para rebuffering
        ClientHttpRequestFactory bufferingFactory = new BufferingClientHttpRequestFactory(basicRequestFactory);

        // Retorna o RestTemplate configurado com o BufferingClientHttpRequestFactory
        return new RestTemplate(bufferingFactory);
    }
}
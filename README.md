Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore
## Descrição do Projeto

Este projeto implementa um sistema de busca e recomendação de produtos para um catálogo de e-commerce fictício chamado MegaStore. A solução foi desenvolvida em Rust com foco em estruturas de dados eficientes, organização modular e boas práticas de programação. O sistema permite buscar produtos por nome e também visualizar recomendações baseadas em relações entre produtos utilizando grafos simples.

## Funcionalidades
Cadastro de produtos em catálogo
Busca de produtos por nome (case-insensitive)
Listagem de todos os produtos
Sistema simples de recomendação baseado em grafo
Estrutura modular em Rust
Testes automatizados

# Tecnologias Utilizadas
Rust
Cargo (gerenciador de build)
HashMap (estrutura de dados)
HashSet (estrutura de grafo)
Testes com cargo test

## Arquitetura do Sistema

O projeto é dividido em módulos: produto.rs define a estrutura dos produtos, catalogo.rs gerencia o armazenamento utilizando HashMap, busca.rs implementa a lógica de busca por nome, recommendation.rs implementa o grafo de recomendações e main.rs controla a execução principal do sistema. Essa divisão facilita manutenção e organização do código.

## Estruturas de Dados

O sistema utiliza HashMap para armazenar os produtos de forma eficiente, permitindo acesso rápido aos dados. Também utiliza HashSet no módulo de recomendação para representar conexões entre produtos em formato de grafo.

## Como executar o projeto

Para executar o sistema, utilize o comando: cargo run

# Como executar os testes

Para rodar os testes automatizados, utilize: cargo test

# Exemplo de uso

O sistema permite cadastrar produtos no catálogo, realizar buscas por nome como “iphone” e visualizar recomendações de produtos relacionados com base nas conexões do grafo.

## Desempenho e Escalabilidade

O uso de HashMap garante inserções e acessos rápidos aos produtos. A estrutura de grafo permite recomendações simples e eficientes. O sistema pode ser expandido para catálogos maiores sem perda significativa de desempenho em buscas diretas.

## Contribuição

Projeto acadêmico desenvolvido para estudo de estruturas de dados e programação em Rust.

## Licença
Este projeto possui finalidade educacional e não possui licença comercial.
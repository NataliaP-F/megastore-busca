# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do Projeto

Este projeto implementa um sistema de busca otimizado para um catálogo de produtos de um e-commerce fictício chamado MegaStore. O objetivo é simular uma estrutura eficiente de armazenamento e busca de produtos utilizando a linguagem Rust, com foco em desempenho, organização de dados e boas práticas de programação.

A solução utiliza a estrutura de dados HashMap para permitir acesso rápido aos produtos e implementação de buscas por nome, com comparação case-insensitive.

## Funcionalidades

- Cadastro de produtos no catálogo
- Armazenamento eficiente utilizando HashMap
- Busca de produtos por nome (case-insensitive)
- Listagem de todos os produtos cadastrados
- Testes unitários para validação da funcionalidade de busca

## Tecnologias Utilizadas

- Linguagem Rust
- Cargo (gerenciador de pacotes e build system)
- Estrutura de dados HashMap
- Testes automatizados com cargo test

## Estrutura do Projeto

src/
- main.rs: execução principal do sistema
- lib.rs: exportação dos módulos do projeto
- produto.rs: definição da estrutura Produto
- catalogo.rs: gerenciamento dos produtos utilizando HashMap
- busca.rs: implementação da lógica de busca

tests/
- busca_test.rs: testes de integração do sistema de busca

## Como Executar o Projeto

Para compilar o projeto:

cargo build

Para executar o sistema:

cargo run

## Como Executar os Testes

Para rodar os testes automatizados:

cargo test

## Arquitetura do Sistema

O sistema é dividido em módulos independentes para facilitar manutenção e escalabilidade. O módulo Produto define a estrutura base dos dados, o módulo Catálogo é responsável pelo armazenamento utilizando HashMap, o módulo Busca implementa a lógica de filtragem dos produtos e o arquivo Main é o ponto de entrada da aplicação.

Essa separação segue boas práticas de organização de código em Rust.

## Estruturas de Dados Utilizadas

Foi utilizada a estrutura HashMap para armazenamento dos produtos, permitindo inserção e acesso eficiente por chave.

A busca é realizada por iteração sobre os valores armazenados, com complexidade O(n), utilizando comparação de strings normalizada para garantir maior precisão.

## Desempenho e Escalabilidade

O uso de HashMap garante eficiência média de O(1) para inserção de dados.

A busca possui complexidade O(n), pois percorre os elementos armazenados. Mesmo assim, o sistema mantém boa escalabilidade para grandes volumes de dados dentro do contexto proposto no projeto.

## Contribuições

Este projeto foi desenvolvido para fins acadêmicos. Sugestões de melhoria podem incluir otimizações na lógica de busca ou expansão das funcionalidades do sistema.

## Licença

Este projeto possui finalidade educacional e não possui licença comercial.

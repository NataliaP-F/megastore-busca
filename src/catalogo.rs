use std::collections::HashMap;
use crate::produto::Produto;

pub struct Catalogo {
    pub produtos: HashMap<u32, Produto>,
}

impl Catalogo {
    pub fn new() -> Self {
        Catalogo {
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.insert(produto.id, produto);
    }

    pub fn listar(&self) {
        for (_, p) in &self.produtos {
            println!("{:?}", p);
        }
    }
}
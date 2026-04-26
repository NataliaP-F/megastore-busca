use crate::catalogo::Catalogo;
use crate::produto::Produto;

pub fn buscar_por_nome(catalogo: &Catalogo, termo: &str) -> Vec<Produto> {
    let mut resultados = Vec::new();

    for produto in catalogo.produtos.values() {
        if produto.nome.to_lowercase().contains(&termo.to_lowercase()) {
            resultados.push(produto.clone());
        }
    }

    resultados
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalogo::Catalogo;
    use crate::produto::Produto;

    #[test]
    fn teste_busca_nome() {
        let mut catalogo = Catalogo::new();

        catalogo.adicionar_produto(Produto {
            id: 1,
            nome: "Mouse Gamer".to_string(),
            categoria: "Periféricos".to_string(),
            marca: "Logitech".to_string(),
        });

        let resultado = buscar_por_nome(&catalogo, "mouse");

        assert_eq!(resultado.len(), 1);
    }
}
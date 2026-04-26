use megastore_busca::catalogo::Catalogo;
use megastore_busca::produto::Produto;
use megastore_busca::busca::buscar_por_nome;

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
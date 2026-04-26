mod produto;
mod catalogo;
mod busca;

use produto::Produto;
use catalogo::Catalogo;
use busca::buscar_por_nome;

fn main() {
    let mut catalogo = Catalogo::new();

    catalogo.adicionar_produto(Produto {
        id: 1,
        nome: "iPhone 15".to_string(),
        categoria: "Eletrônicos".to_string(),
        marca: "Apple".to_string(),
    });

    catalogo.adicionar_produto(Produto {
        id: 2,
        nome: "Galaxy S23".to_string(),
        categoria: "Eletrônicos".to_string(),
        marca: "Samsung".to_string(),
    });

    catalogo.adicionar_produto(Produto {
        id: 3,
        nome: "Camiseta Nike".to_string(),
        categoria: "Vestuário".to_string(),
        marca: "Nike".to_string(),
    });

    println!("=== LISTA COMPLETA ===");
    catalogo.listar();

    println!("\n=== BUSCA: 'iphone' ===");
    let resultados = buscar_por_nome(&catalogo, "iphone");

    for p in resultados {
        println!("{:?}", p);
    }
}

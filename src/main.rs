pub mod produto;
pub mod catalogo;
pub mod recommendation;
pub mod busca;

use produto::Produto;
use catalogo::Catalogo;
use recommendation::RecommendationGraph;
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

    let mut grafo = RecommendationGraph::new();

// criando relações entre produtos (simulando "quem compra também compra")
    grafo.add_relation(1, 2);
    grafo.add_relation(1, 3);

    println!("\n=== RECOMENDAÇÕES PARA PRODUTO 1 ===");
    let recomendados = grafo.recommend(1);

    for id in recomendados {
    println!("Produto relacionado: {}", id);
    }
}

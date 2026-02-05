mod lexer;

fn main() {
  let source = r#"
{
  "usuario": {
    "id": 12345,
    "nome": "João da Silva",
    "email": "joao@email.com",
    "ativo": true,
    "admin": false,
    "ultimo_login": null
  },

  "configuracoes": {
    "tema": "dark",
    "notificacoes": true,
    "volume": 0.75,
    "idiomas": ["pt-BR", "en-US", "es-ES"]
  },

  "historico_compras": [
    {
      "pedido_id": 1,
      "valor": 199.90,
      "pago": true,
      "itens": [
        { "nome": "Teclado", "quantidade": 1 },
        { "nome": "Mouse", "quantidade": 2 }
      ]
    },
    {
      "pedido_id": 2,
      "valor": 89.50,
      "pago": false,
      "itens": []
    }
  ],

  "estatisticas": {
    "pontuacoes": [10, 20, 30, 40.5],
    "media": 25.125,
    "melhor_resultado": 40.5
  },

  "tags": ["json", "parser", "lexer", "teste"],

  "metadata": {
    "criado_em": "2026-02-05T12:00:00Z",
    "atualizado_em": null
  }
}

  "#;
  let mut lexer = lexer::Lexer::new(source);

  lexer.display();
}

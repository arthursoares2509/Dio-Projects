import json

# Base fictícia de trilhas
trilhas = {
    "1": {"nome": "Trilha IBM Bob IA", "conteudo": "Aprenda sobre contexto, prompts e boas práticas de IA."},
    "2": {"nome": "Trilha Python Basico", "conteudo": "Estrutura de dados, funções e lógica de programação."}
}

def listar_trilhas():
    print("\n--- Trilhas Disponíveis ---")
    for key, item in trilhas.items():
        print(f"[{key}] {item['nome']}")

def consultar_trilha(id_trilha):
    trilha = trilhas.get(id_trilha)
    if trilha:
        print(f"\nConteúdo da {trilha['nome']}:\n{trilha['conteudo']}")
    else:
        print("\nTrilha não encontrada.")

def gerar_certificado(nome_usuario, id_trilha):
    trilha = trilhas.get(id_trilha)
    if trilha:
        print(f"\n=== CERTIFICADO DE CONCLUSÃO ===")
        print(f"Certificamos que {nome_usuario} concluiu a {trilha['nome']}.")
        print("================================")

if __name__ == "__main__":
    listar_trilhas()
    opcao = input("\nDigite o número da trilha para acessar: ")
    consultar_trilha(opcao)
    
    nome = input("\nDigite seu nome para gerar o certificado: ")
    gerar_certificado(nome, opcao)
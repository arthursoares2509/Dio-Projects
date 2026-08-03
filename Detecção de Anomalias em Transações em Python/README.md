\# Detecção de Anomalias em Transações Financeiras com Python (Desafio DIO)



Projeto prático desenvolvido para o desafio de Machine Learning e Ciência de Dados da DIO, focado na identificação de fraudes e comportamentos atípicos em transações de cartão/banco.



\## 📌 Abordagem e Metodologia

Para resolver o problema de detecção de fraudes, foram aplicadas duas estratégias principais:



1\. \*\*Aprendizado Não Supervisionado (Isolation Forest):\*\* Identificação de desvios do padrão do conjunto de dados sem dependência prévia de rótulos de fraude. Ideal para identificar novos padrões de ataques.

2\. \*\*Aprendizado Supervisionado (Random Forest Classifier):\*\* Treinamento do modelo baseado em histórico rotulado para classificação precisa de transações legítimas vs. fraudulentas.



\## 🛠️ Tecnologias Utilizadas

\- \*\*Python\*\*

\- \*\*Pandas \& NumPy\*\* (Tratamento de Dados)

\- \*\*Scikit-Learn\*\* (Pré-processamento, Isolation Forest, Random Forest e Métricas)



\## 🚀 Como Executar

1\. Instale as dependências necessárias:

&#x20;  ```bash

&#x20;  pip install -r requirements.txt


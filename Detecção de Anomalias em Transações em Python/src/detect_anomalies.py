import pandas as pd
import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.ensemble import IsolationForest, RandomForestClassifier
from sklearn.metrics import classification_report, confusion_matrix
from sklearn.preprocessing import StandardScaler

def generate_synthetic_data(num_records=1000):
    """Gera um conjunto de dados simulado de transações financeiras."""
    np.random.seed(42)
    
    # Transações normais
    amounts_normal = np.random.normal(loc=150, scale=50, size=int(num_records * 0.95))
    times_normal = np.random.uniform(0, 24, size=int(num_records * 0.95))
    is_fraud_normal = np.zeros(int(num_records * 0.95))
    
    # Transações anômalas/fraudes (valores fora do padrão)
    amounts_fraud = np.random.normal(loc=3000, scale=800, size=int(num_records * 0.05))
    times_fraud = np.random.uniform(1, 4, size=int(num_records * 0.05)) # madrugada
    is_fraud_fraud = np.ones(int(num_records * 0.05))
    
    df_normal = pd.DataFrame({'valor': amounts_normal, 'hora': times_normal, 'is_fraude': is_fraud_normal})
    df_fraud = pd.DataFrame({'valor': amounts_fraud, 'hora': times_fraud, 'is_fraude': is_fraud_fraud})
    
    df = pd.concat([df_normal, df_fraud]).sample(frac=1, random_state=42).reset_index(drop=True)
    return df

def run_anomaly_detection():
    print("--- 1. Gerando/Carregando Dados de Transações ---")
    df = generate_synthetic_data(2000)
    print(f"Total de registros: {len(df)} | Fraudes reais: {int(df['is_fraude'].sum())}")
    
    features = ['valor', 'hora']
    X = df[features]
    y = df['is_fraude']
    
    # Padronização das features
    scaler = StandardScaler()
    X_scaled = scaler.fit_transform(X)
    
    print("\n--- 2. Detecção Não Supervisionada (Isolation Forest) ---")
    # Contaminação estimada em 5%
    iso_forest = IsolationForest(contamination=0.05, random_state=42)
    df['anomaly_iso'] = iso_forest.fit_predict(X_scaled)
    # Isolation Forest retorna -1 para anomalia e 1 para normal
    df['pred_iso'] = df['anomaly_iso'].map({1: 0, -1: 1})
    
    print("Resultado Isolation Forest:")
    print(confusion_matrix(y, df['pred_iso']))
    
    print("\n--- 3. Detecção Supervisionada (Random Forest) ---")
    X_train, X_test, y_train, y_test = train_test_split(X_scaled, y, test_size=0.3, random_state=42, stratify=y)
    
    rf = RandomForestClassifier(n_estimators=100, random_state=42)
    rf.fit(X_train, y_train)
    
    y_pred = rf.predict(X_test)
    print("Relatório de Classificação (Random Forest):")
    print(classification_report(y_test, y_pred))

if __name__ == "__main__":
    run_anomaly_detection()
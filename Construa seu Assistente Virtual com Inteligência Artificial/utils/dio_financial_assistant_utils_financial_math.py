import pandas as pd

def calculate_compound_interest(principal, monthly_contribution, annual_rate, years):
    months = years * 12
    monthly_rate = (1 + annual_rate / 100) ** (1 / 12) - 1
    
    data = []
    total_invested = principal
    current_amount = principal
    
    for month in range(1, months + 1):
        current_amount = (current_amount + monthly_contribution) * (1 + monthly_rate)
        total_invested += monthly_contribution
        
        if month % 12 == 0 or month == months:
            year = month // 12
            data.append({
                "Ano": year,
                "Mes": month,
                "Total Investido": round(total_invested, 2),
                "Montante Final": round(current_amount, 2),
                "Juros Acumulados": round(current_amount - total_invested, 2)
            })
            
    return pd.DataFrame(data)

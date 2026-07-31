import os
from google import genai
from google.genai import types

def get_gemini_response(history, user_input, api_key):
    if not api_key:
        return "Erro: Chave de API do Gemini não configurada. Defina a variável de ambiente GEMINI_API_KEY ou insira na barra lateral."
    
    try:
        client = genai.Client(api_key=api_key)
        
        system_instruction = (
            "Você é um consultor financeiro especialista em experiência do usuário, "
            "comunicação clara e educação financeira. Seu objetivo é auxiliar clientes "
            "com FAQs inteligentes, explicações de produtos financeiros (CDB, Tesouro Direto, Fundos) "
            "e orientações práticas. Mantenha as respostas diretas, objetivas, sem jargões desnecessários "
            "e siga estritamente boas práticas de UX. Nunca recomende investimentos de alto risco sem aviso prévio."
        )
        
        contents = []
        for msg in history:
            role = "user" if msg["role"] == "user" else "model"
            contents.append(types.Content(
                role=role,
                parts=[types.Part.from_text(text=msg["content"])]
            ))
            
        contents.append(types.Content(
            role="user",
            parts=[types.Part.from_text(text=user_input)]
        ))
        
        config = types.GenerateContentConfig(
            system_instruction=system_instruction,
            temperature=0.3,
        )
        
        response = client.models.generate_content(
            model="gemini-2.5-flash",
            contents=contents,
            config=config,
        )
        
        return response.text
    except Exception as e:
        return f"Erro ao comunicar com a IA: {str(e)}"

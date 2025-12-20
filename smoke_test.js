// smoke_test.js
require('dotenv').config();

async function testGroq() {
    console.log("🚀 Iniciando Smoke Test de Groq...");
    
    if (!process.env.GROQ_API_KEY) {
        console.error("❌ ERROR: No encontré la variable GROQ_API_KEY en tu .env");
        return;
    }

    try {
        const response = await fetch("https://api.groq.com/openai/v1/chat/completions", {
            method: "POST",
            headers: {
                "Authorization": `Bearer ${process.env.GROQ_API_KEY}`,
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                model: "llama-3.1-8b-instant",
                messages: [{ role: "user", content: "Hola, ¿estás configurado correctamente?" }]
            })
        });

        const data = await response.json();
        
        if (data.choices) {
            console.log("✅ ÉXITO: Groq respondió correctamente:");
            console.log("🤖 IA:", data.choices[0].message.content);
        } else {
            console.error("❌ ERROR en la respuesta:", data);
        }
    } catch (err) {
        console.error("❌ ERROR de conexión:", err.message);
    }
}

testGroq();
import { createClient } from '@supabase/supabase-client';

export default async function handler(req, res) {
  if (req.method !== 'POST') return res.status(405).send('Method not allowed');

  const { pregunta } = req.body;

  // 1. Configuración de clientes
  const supabase = createClient(process.env.SUPABASE_URL, process.env.SUPABASE_KEY);

  try {
    // 2. Obtener el Embedding de la pregunta (Usando el mismo modelo que en Rust)
    const hfResponse = await fetch(
      "https://api-inference.huggingface.co/pipeline/feature-extraction/sentence-transformers/all-MiniLM-L6-v2",
      {
        headers: { Authorization: `Bearer ${process.env.HUGGINGFACE_KEY}` },
        method: "POST",
        body: JSON.stringify({ inputs: pregunta }),
      }
    );
    const embedding = await hfResponse.json();

    // 3. Búsqueda semántica en Supabase (RPC)
    const { data: documentos, error } = await supabase.rpc('match_documents', {
      query_embedding: embedding,
      match_threshold: 0.5,
      match_count: 3,
    });

    if (error) throw error;

    const contexto = documentos.map(d => d.contenido).join("\n---\n");

    // 4. Preguntar a Groq con el contexto de tus archivos
    const groqResponse = await fetch("https://api.groq.com/openai/v1/chat/completions", {
      method: "POST",
      headers: {
        "Authorization": `Bearer ${process.env.GROQ_API_KEY}`,
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        model: "llama-3.1-8b-instant",
        messages: [
          { role: "system", content: `Eres un asistente experto. Usa este contexto para responder: ${contexto}` },
          { role: "user", content: pregunta }
        ]
      })
    });

    const groqData = await groqResponse.json();
    
    res.status(200).json({ 
      respuesta: groqData.choices[0].message.content 
    });

  } catch (err) {
    res.status(500).json({ error: err.message });
  }
}
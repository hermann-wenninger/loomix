import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Apikeys {
  GROQ_KEY: string;
  MISTRAL_KEY: string;
  OLLAMA_KEY: string;
  OPENAI_KEY: string;
  OPENROUTER_KEY: string;
  ANTROPIC_KEY: string;
  COHERCE_KEY: string;
  DEEPSEEK_KEY: string;
  GOOGLE_KEY: string;
  IIELEVEN_KEY: string;
  SERPAPI_KEY: string;
}
 
export default function Keys() {
  const [apikeys, setApikeys] = useState<Apikeys | null>(null);

  useEffect(() => {
    invoke<Apikeys>("get_keys").then(setApikeys);
  }, []);

  return (
    <div style={{ padding: 20 }}>
      <h2>Rust → React Daten</h2>
      {apikeys ? (
        <ul>
          <li><b>Ollama:</b> {apikeys.OLLAMA_KEY}</li>
          <li><b>Groq:</b> {apikeys.GROQ_KEY}</li>
          <li><b>Mistral:</b> {apikeys.MISTRAL_KEY}</li>
          <li><b>Openai:</b> {apikeys.OPENAI_KEY}</li>
          <li><b>Openrouter:</b> {apikeys.OPENROUTER_KEY}</li>
          <li><b>Antropic:</b> {apikeys.ANTROPIC_KEY}</li>
          <li><b>Coherc:</b> {apikeys.COHERCE_KEY}</li>
          <li><b>Deepseek:</b> {apikeys.DEEPSEEK_KEY}</li>
          <li><b>Google:</b> {apikeys.GOOGLE_KEY}</li>
          <li><b>IIeleven:</b> {apikeys.IIELEVEN_KEY}</li>
          <li><b>SERPAPI:</b> {apikeys.SERPAPI_KEY}</li>
        </ul>
      ) : (
        <p>Lade Daten aus Rust ...</p>
      )}
    </div>
  );
}

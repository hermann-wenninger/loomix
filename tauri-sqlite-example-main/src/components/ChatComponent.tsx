import { useEffect,useState } from "react";
import { invoke } from "@tauri-apps/api/core";


interface ChatMessage{
    role:string;
    content:string;
}

interface ChatResponse{
    message:ChatMessage;
}

const ChatComponent : React.FC = () => {
    const [messages, setMessages] = useState<ChatMessage[]>([]);
    const [input, setInput] = useState<string>("");
    const [models, setModels] = useState<string[]>([]); 
    const [selectedModel, setSelectedModel] = useState<string>("");
   
    useEffect(() => {
        const fetchedModels = async () => {
            try {
                const fetchedModels : string[] = await invoke("get_models");
                setModels(fetchedModels);
                if (fetchedModels.length > 0) {
                    setSelectedModel(fetchedModels[0]);
                }
            } catch (error) {
                console.error("Fehler beim Abrufen der Modelle:", error);
            }

    }

    export default ChatComponent;
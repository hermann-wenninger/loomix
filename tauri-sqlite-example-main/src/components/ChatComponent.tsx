import { useEffect,useState, ChangeEvent, FormEvent } from "react";
import { invoke, Channel } from "@tauri-apps/api/core";


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
        };
        fetchedModels();
    }, []);

    const sendMessage = async (e:FormEvent) :Promise<void> => {
    e.preventDefault(); 
        if (!input.trim()||!selectedModel ) return;
        const userMessage:ChatMessage = { role: "user", content: input.trim() };
        setMessages(prev => [...prev, userMessage]);
        setInput("")

        const channel = new Channel<ChatResponse>();

        channel.onmessage = (data:ChatResponse) => {
            const messageContent = data.message;
            setMessages(prev =>{
                const lastMsg =  prev[prev.length -1];
                if(lastMsg && lastMsg.role === "assistant"){
                    return [
                        ...prev.slice(0, -1),
                        { role: "assistant", content: lastMsg.content + messageContent }
                    ];
                }
                return [...prev, { role: "assistant", content: messageContent } ];
            });

        };
        try {
            await invoke("chat", { 
                request:{
                model: selectedModel, 
                messages: [...messages,userMessage],
            },
            onStream: channel,
        });
        } catch (error) {
            console.error("Fehler beim Senden der Nachricht:", error);
            setMessages(prev => [
                ...prev,
                 { role: "assistant", content: "Fehler beim Senden der Nachricht." }
                ]);
        }
};
const handleInputChange =(e:ChangeEvent<HTMLInputElement>) :void => {
    setInput(e.target.value);   
};
const handleSelectChange = (e:ChangeEvent<HTMLSelectElement>) :void => {
    setSelectedModel(e.target.value);
};
    return (
        <div className="chat-container">
            <div className="chat-messages">

         
            {messages.map((msg, index) => (
                <div key={index} className={`message ${msg.role}`}>
                  <span>{msg.role}</span>
                    <p>{msg.content}</p>
                </div>
            ))}
            </div>
            <form onSubmit={sendMessage}>
                <select id="inline-model-select"
                 value={selectedModel} 
                 onChange={handleSelectChange}>
                {models.map((model) => (
                <option key={model} value={model}>
                    {model}
                </option>
                    ))}
                </select>
                <input
                    type="text"
                    value={input}
                    onChange={handleInputChange}
                    placeholder="Type your message..."
                />
                <button type="submit">Send</button>
           </form>
            </div>
    );
}
    export default ChatComponent;
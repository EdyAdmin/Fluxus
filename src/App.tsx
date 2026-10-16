import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Message {
  role: "User" | "Assistant" | "Manager" | "System";
  content: string;
  model: string | null;
  timestamp?: string;
}
function App() {
  const [inputText, setImputText] = useState("");
  const [messages, setMessages] = useState<Message[]>([]);

  async function handle_send() {
    if (!inputText.trim()) return;

    const reloj = new Date();
    const horaLimpia = reloj.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });

    try {
      const newMsg = await invoke<Message>("send_message", {
        content: inputText,
      });

      const mensajeConHora = {
      ...newMsg, 
      timestamp: horaLimpia
      };

      setMessages((prev) => [...prev,mensajeConHora]);
      setImputText("");
    } catch (error) {
      console.error("Error al llamar a Rust: ", error);
    }
  }

  return (
    <>
      <main className="container">
        <div className="Menu">

        </div>

        <div className="mainChat">
          <div className="historial-box">
            {messages.map((msg, index) => (
              <div
                key={index}
                className={`celda ${msg.role === "User" ? "user-message" : "ai-message"}`}
              >
                <p>
                 {msg.content}
                </p>
                
                <span className="timestamp">
                  {msg.timestamp}
                </span> 
              </div>
            ))}
          </div>

          <div className="chat-box">
            <form
            className="row"
            onSubmit={(e) => {
              e.preventDefault();
              handle_send();
            }}
            >
              <input
                id="message_area"
                value={inputText}
                onChange={(e) => setImputText(e.currentTarget.value)}
                placeholder="Enter your text"
              />
              <button type="submit">send</button>
            </form>

          </div>
          
        </div>

      </main>
    </>
  );
}

export default App;

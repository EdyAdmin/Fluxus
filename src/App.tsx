import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Message {
  role: "User" | "Assistant" | "Manager" | "System";
  content: string;
  model: string | null;
  timestamp: number;
}
function App() {
  const [inputText, setImputText] = useState("");
  const [messages, setMessages] = useState<Message[]>([]);

  function formatTimestamp(timestamp: number): string {
    return new Date(timestamp).toLocaleTimeString([], {
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  async function handle_send() {
    if (!inputText.trim()) return;

    try {
      const newMsg = await invoke<Message>("send_message", {
        content: inputText,
      });

      setMessages((prev) => [...prev,newMsg]);
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
                  {formatTimestamp(msg.timestamp)}
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

import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Message {
  role: "User" | "Assistant" | "Manager" | "System";
  content: string;
  model: string | null;
}
function App() {
  const [inputText, setImputText] = useState("");
  const [messages, setMessages] = useState<Message[]>([]);

  async function handle_send() {
    if (!inputText.trim()) return;
    try {
      const newMsg = await invoke<Message>("send_message", {
        content: inputText,
      })
      setMessages((prev) => [...prev,newMsg]);
      setImputText("");
    } catch (error) {
      console.error("Error al llamar a Rust: ", error);
    }
  }

  return (
    <>
      <main className="container">
        <h1>Welcome to Fluxus</h1>
        <h2> Write your messages </h2>

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
        <div className="chat-box">
          {messages.map((msg, index) => (
            <p key={index}>
              <strong>[{msg.role}]:</strong> {msg.content}
            </p>
          ))}
        </div>
      </main>
      <footer> Contact to <a href= "https://www.linkedin.com/in/eduard-andrei-szabo-02406641b/"> Eduard Andrei Szabo </a> for more information </footer>
    </>
  );
}

export default App;

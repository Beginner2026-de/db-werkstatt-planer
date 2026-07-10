const { invoke } = window.__TAURI__.core;
const BACKEND_IP = "http://127.0.0.1:8888";

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

document.getElementById("loginForm").addEventListener("submit", async (event) => {
    event.preventDefault(); // Verhindert das Neuladen der Seite

    // Formulardaten auslesen
    const formData = new FormData(event.target);
    const data = Object.fromEntries(formData.entries());

    try {
        // Daten per POST an die dynamische IP senden
        const response = await fetch(`${BACKEND_IP}/api/auth/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        });

        if (response.ok) {
            const result = await response.json();
            console.log("Erfolgreich eingeloggt:", result);
        } else {
            console.error("Fehler beim Login");
        }
    } catch (error) {
        console.error("Netzwerkfehler:", error);
    }
});

document.getElementById("registerForm").addEventListener("submit", async (event) => {
    event.preventDefault(); // Verhindert das Neuladen der Seite

    // Formulardaten auslesen
    const formData = new FormData(event.target);
    const data = Object.fromEntries(formData.entries());

    try {
        // Daten per POST an die dynamische IP senden
        const response = await fetch(`${BACKEND_IP}/api/auth/register`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        });

        if (response.ok) {
            const result = await response.json();
            console.log("Erfolgreich in die Warteliste eingetragen:", result);
        } else {
            console.error("Fehler beim Registrieren");
        }
    } catch (error) {
        console.error("Netzwerkfehler:", error);
    }
});
async function loadComponent(elementId, filePath) {
    const element = document.getElementById(elementId);
    if (!element) return;

    const response = await fetch(filePath);
    if (!response.ok) {
        throw new Error(`Failed to load ${filePath}: ${response.status}`);
    }

    element.innerHTML = await response.text();
}

function initComponents() {
    loadComponent("header", "components/header.html");
    loadComponent("footer", "components/footer.html");
}

if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", initComponents);
} else {
    initComponents();
}
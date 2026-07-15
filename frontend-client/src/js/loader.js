async function loadHeader() {
    const response = await fetch("components/header.html");
    const html = await response.text();

    document.getElementById("header").innerHTML = html;
}

async function loadFooter() {
    const response = await fetch("components/footer.html");
    const html = await response.text();

    document.getElementById("footer").innerHTML = html;
}
loadHeader();
loadFooter();
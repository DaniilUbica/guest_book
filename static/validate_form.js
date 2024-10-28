function validateForm() {
    const mail = document.getElementById('mail').value.trim();
    const login = document.getElementById('login').value.trim();
    const comment = document.getElementById('comment').value.trim();

    if (mail === "" || login === "" || comment === "") {
        alert("All fields should not be empty");
        return false;
    }

    location.href = '/';
    return true;
}

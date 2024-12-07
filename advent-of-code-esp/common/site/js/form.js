document.addEventListener("DOMContentLoaded", () => {
    // Countdown management for multiple forms
    const manageCountdown = (formId, countdownId, resultId) => {
        const form = document.getElementById(formId);
        const countdownElement = document.getElementById(countdownId);
        const resultElement = document.getElementById(resultId);
        let countdown = parseInt(form.dataset.countdown, 10); // Read initial countdown value from the data attribute
        let countdownInterval;

        form.addEventListener("submit", async (event) => {
            event.preventDefault(); // Prevent form submission

            // Reset and start the countdown
            clearInterval(countdownInterval);
            countdown = parseInt(form.dataset.countdown, 10); // Reset the countdown to the original value
            countdownElement.textContent = `Waiting for Result: ${countdown} seconds`;
            resultElement.innerHTML = ""

            countdownInterval = setInterval(() => {
                countdown -= 1;
                if (countdown >= 0) {
                    countdownElement.textContent = `Waiting for Result: ${countdown} seconds`;
                } else {
                    clearInterval(countdownInterval);
                    countdownElement.textContent = "Time is up!";
                }
            }, 1000);

            // Simulate server request
            try {
                const response = await fetch(form.action, {
                    method: form.method,
                    body: new FormData(form),
                });

                if (response.ok) {
                    clearInterval(countdownInterval);
                    const data = await response.json();
                    if (data.complete === 1) {
                        resultElement.innerHTML = `&#9989; Success, Runtime ${data.runtime}, Minimum Free Heap Size [${data.free_heap_size_before}, ${data.free_heap_size_after}]`;
                    } else if (data.complete === -1) {
                        resultElement.innerHTML = `&#10060; Fail, Runtime ${data.runtime}, Minimum Free Heap Size [${data.free_heap_size_before}, ${data.free_heap_size_after}]`;
                    }
                    countdownElement.textContent = "";
                } else {
                    throw new Error("Server error");
                }
            } catch (error) {
                clearInterval(countdownInterval);
                countdownElement.textContent = "Error waiting for result.";
            }
        });
    };

    // File upload handling and validation
    const setupFileUpload = (formId, fileInputId, chosenTextId, maxSizeMB) => {
        const fileInput = document.getElementById(fileInputId);
        const chosenText = document.getElementById(chosenTextId);

        // Update chosen file name
        fileInput.addEventListener("change", function () {
            if (this.files && this.files.length > 0) {
                chosenText.textContent = " - " + this.files[0].name;
            }
        });

        // Validate file size on submit
        document.getElementById(formId).addEventListener("submit", function (event) {
            const file = fileInput.files[0];
            const maxSize = maxSizeMB * 1024 * 1024; // Convert MB to bytes

            if (file && file.size > maxSize) {
                alert(`The file is too large. Please select a file smaller than ${maxSizeMB}MB.`);
                event.preventDefault();
            }
        });
    };

    // Initialize functionality for Form 1
    manageCountdown("puzzle-upload-form-1", "countdown-1", "result-1");
    setupFileUpload("puzzle-upload-form-1", "puzzle-upload-1", "puzzle-chosen-1", 1);

    // Initialize functionality for Form 2
    manageCountdown("puzzle-upload-form-2", "countdown-2", "result-2");
    setupFileUpload("puzzle-upload-form-2", "puzzle-upload-2", "puzzle-chosen-2", 1);
});
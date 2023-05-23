function formatFileSize(bytes) {
    const units = ["b", "kb", "mb", "gb", "tb"];
    const kib = 1024.0;

    if (bytes === 0 || isNaN(bytes)) {
        return "-";
    }

    let base = Math.floor(Math.log(bytes) / Math.log(kib));
    if (base >= units.length) {
        return "Too Large";
    }

    let adjustedBytes = bytes / Math.pow(kib, base);
    let unit = units[base];

    if (Math.round((adjustedBytes % 1) * 10) === 0) {
        return `${Math.floor(adjustedBytes)} <span>${unit}</span>`;
    } else {
        return `${adjustedBytes.toFixed(1)} <span>${unit}</span>`;
    }
}

function formatDate(dateString) {
    // Parse the date
    let date = new Date(dateString);

    // Calculate the difference between the current time and the given time
    let diffInSeconds = Math.floor((new Date() - date) / 1000);

    // Check if it is less than a minute
    if (diffInSeconds < 60) {
        return "just now";
    }

    // Check if it is less than an hour
    if (diffInSeconds < 3600) {
        return Math.floor(diffInSeconds / 60) + " minutes ago";
    }

    // Check if it is less than a day
    if (diffInSeconds < 86400) {
        return Math.floor(diffInSeconds / 3600) + " hours ago";
    }

    // Otherwise, show it in days
    return Math.floor(diffInSeconds / 86400) + " days ago";
}


function formatTableRows() {
    let sizeCells = document.querySelectorAll('td.size');
    for (let cell of sizeCells) {
        let originalValue = parseInt(cell.textContent);
        cell.setAttribute('aria-label', originalValue);
        let formattedValue = formatFileSize(originalValue);
        cell.innerHTML = formattedValue;
    }

    // Find all table cells with the class "modified"
    let dateCells = document.querySelectorAll('td.modified');
    for (let cell of dateCells) {
        let originalDate = cell.textContent;
        cell.setAttribute('aria-label', originalDate);
        let localDate = new Date(originalDate).toLocaleString();
        let formattedDate = formatDate(localDate);
        cell.innerHTML = formattedDate;
    }
}

function first_run_manifest() {
    formatTableRows()
}

document.addEventListener('DOMContentLoaded', first_run_manifest);
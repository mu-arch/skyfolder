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

function formatTimeAgo(unixTimestamp) {
    const currentTime = Math.floor(Date.now() / 1000); // Convert current time to Unix timestamp
    const timeDifference = currentTime - unixTimestamp; // Calculate the time difference in seconds

    if (timeDifference < 60) {
        return "just now";
    } else if (timeDifference < 3600) {
        const minutes = Math.floor(timeDifference / 60);
        return minutes === 1 ? "1 minute ago" : `${minutes} minutes ago`;
    } else if (timeDifference < 86400) {
        const hours = Math.floor(timeDifference / 3600);
        return hours === 1 ? "1 hour ago" : `${hours} hours ago`;
    } else {
        const days = Math.floor(timeDifference / 86400);
        return days === 1 ? "1 day ago" : `${days} days ago`;
    }
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
        let formattedDate = formatTimeAgo(originalDate);
        cell.innerHTML = formattedDate;
    }
}

function first_run_manifest() {
    formatTableRows()
}

document.addEventListener('DOMContentLoaded', first_run_manifest);
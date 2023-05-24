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
    const currentTime = Math.floor(Date.now() / 1000);
    const timeDifference = currentTime - unixTimestamp;

    if (timeDifference < 60) return "just now";
    if (timeDifference < 3600) return `${Math.floor(timeDifference / 60)} minutes ago`;
    if (timeDifference < 86400) return `${Math.floor(timeDifference / 3600)} hours ago`;
    return `${Math.floor(timeDifference / 86400)} days ago`;
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

function preloadNextPage(element) {
    const anchorElement = element.querySelector('a');
    const linkHref = anchorElement.href;

    if (anchorElement && anchorElement.rel !== 'preload') {
        if (linkHref.endsWith('/') && !anchorElement.hasAttribute('data-preload')) {
            fetch(linkHref, { method: 'GET' }); // Preload the resource
            anchorElement.setAttribute('data-preload', 'true');
        }
    }
}

function navurl(url) {
    window.location.href = url;
}


function first_run_manifest() {
    formatTableRows()
}

document.addEventListener('DOMContentLoaded', first_run_manifest);
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


function formatTableRows() {
    // Find all table cells with the class "size"
    let cells = document.querySelectorAll('td.size');

    for (let cell of cells) {
        // Get the original value
        let originalValue = parseInt(cell.textContent);

        // Store the original value in an aria-label attribute
        cell.setAttribute('aria-label', originalValue);

        // Format the value
        let formattedValue = formatFileSize(originalValue);

        // Replace the cell content with the formatted value
        cell.innerHTML = formattedValue;
    }
}

function first_run_manifest() {
    formatTableRows()
}

window.onload = function() {
    first_run_manifest()
}

const logKib = Math.log(1024.0);
const units = ["b", "kb", "mb", "gb", "tb"];

function formatFileSize(bytes) {
    if (bytes === 0 || isNaN(bytes)) {
        return "-";
    }

    let base = Math.floor(Math.log(bytes) / logKib);
    if (base >= units.length) {
        return "Too Large";
    }

    let adjustedBytes = bytes / Math.pow(1024.0, base);
    let unit = units[base];

    return (Math.round((adjustedBytes % 1) * 10) === 0)
        ? `${Math.floor(adjustedBytes)} <span>${unit}</span>`
        : `${adjustedBytes.toFixed(1)} <span>${unit}</span>`;
}

function formatTimeAgo(unixTimestamp) {
    const currentTime = Math.floor(Date.now() / 1000);
    const timeDifference = currentTime - unixTimestamp;

    if (timeDifference < 60) return "just now";

    const minutes = Math.floor(timeDifference / 60);
    if (timeDifference < 3600) {
        return `${minutes} ${minutes === 1 ? "minute" : "minutes"} ago`;
    }

    const hours = Math.floor(minutes / 60);
    if (timeDifference < 86400) {
        return `${hours} ${hours === 1 ? "hour" : "hours"} ago`;
    }

    const days = Math.floor(hours / 24);
    return `${days} ${days === 1 ? "day" : "days"} ago`;
}


function formatTableRows() {
    const table = document.querySelector('#table');
    const rows = table.querySelectorAll('tbody tr');
    const anchors = table.getElementsByTagName('a');

    let fragment = document.createDocumentFragment();
    if (rows.length === 0) {
        const emptyDiv = document.createElement('div');
        emptyDiv.classList.add('folder-empty');
        emptyDiv.innerHTML = '<div></div><span>This folder is empty.</span>';
        fragment.appendChild(emptyDiv);
    }

    // Rest of the code...

    table.parentNode.appendChild(fragment);
}

function formatTableRows() {
    const table = document.querySelector('#table');
    const rows = table.querySelectorAll('tbody tr');
    const anchors = table.getElementsByTagName('a');

    let fragment = document.createDocumentFragment();

    if (rows.length === 0) {
        const emptyDiv = document.createElement('div');
        emptyDiv.classList.add('folder-empty');
        emptyDiv.innerHTML = '<div></div><span>This folder is empty.</span>';
        fragment.appendChild(emptyDiv);
    } else {
        for (let i = 0; i < rows.length; i++) {
            let row = rows[i];

            // format size
            let sizeCell = row.children[2];
            let originalSize = parseInt(sizeCell.textContent);
            sizeCell.setAttribute('aria-label', originalSize);
            sizeCell.innerHTML = formatFileSize(originalSize);

            // format date
            let dateCell = row.children[1];
            let originalDate = dateCell.textContent;
            dateCell.setAttribute('aria-label', originalDate);
            dateCell.textContent = formatTimeAgo(originalDate);

            // set onclick event for row
            let anchor = anchors[i];
            let href = anchor.getAttribute('href');
            anchor.parentNode.setAttribute('onclick', `navurl('${href}')`);

            // choose icon
            let firstTd = row.getElementsByTagName('td')[0];
            let firstDiv = firstTd.getElementsByTagName('div')[0];

            let name = firstTd.innerText.toLowerCase();
            let isDir = name.endsWith('/');
            let positionText;

            if (isDir) {
                positionText = "-128px 0px";
            } else {
                let extension = name.split('.').pop();

                // mapping of file extensions to icon positions
                const iconPositions = {
                    'rs': "0px -128px",
                    'iso': "-384px 0px",
                    'json': "-512px 0px",
                    'js': "-512px 0px",
                    'py': "-640px 0px",
                    'zip': "-768px 0px",
                    'gz': "-768px 0px",
                    'rar': "-768px 0px",
                    '7z': "-768px 0px",
                    'tar': "-768px 0px",
                    'bz2': "-768px 0px",
                    'xz': "-768px 0px",
                    'pdf': "-896px 0px",
                    'jpg': "-512px -128px",
                    'jpeg': "-512px -128px",
                    'svg': "-384px -128px",
                    'png': "-640px -128px",
                    'gif': "-896px -128px",
                    'ds_store': "-768px -128px",
                    'default': "-256px 0px",
                };

                positionText = iconPositions[extension] || iconPositions['default'];
            }

            firstDiv.style.backgroundPosition = positionText;
        }
    }

    table.parentNode.insertBefore(fragment, table.nextSibling);
}



function set_table_name_row_onclick() {
    var table = document.getElementById('table');
    var anchors = table.getElementsByTagName('a');
    for (var i = 0; i < anchors.length; i++) {
        var anchor = anchors[i];
        var href = anchor.getAttribute('href');
        anchor.parentNode.setAttribute('onclick', 'navurl(\'' + href + '\')');
    }
}


function table_empty_display() {
    const table = document.querySelector('#table');

    if (table.querySelectorAll('tbody tr').length === 0) {
        const emptyDiv = document.createElement('div');
        emptyDiv.classList.add('folder-empty');
        emptyDiv.innerHTML = '<div></div><span>This folder is empty.</span>';
        table.parentNode.insertBefore(emptyDiv, table.nextSibling);
    }
}



function navurl(url) {
    window.location.href = url;
}

//onmouseover="preloadNextPage(this)"

/*function preloadNextPage(element) {
    const anchorElement = element.querySelector('a');
    const linkHref = anchorElement.href;

    if (anchorElement && anchorElement.rel !== 'preload') {
        if (linkHref.endsWith('/') && !anchorElement.hasAttribute('data-preload')) {
            fetch(linkHref, { method: 'GET' }); // Preload the resource
            anchorElement.setAttribute('data-preload', 'true');
        }
    }
}
*/

function first_run_manifest() {
    formatTableRows()
}

document.addEventListener('DOMContentLoaded', first_run_manifest);
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
    if (timeDifference < 3600) {
        const minutes = Math.floor(timeDifference / 60);
        return `${minutes} ${minutes === 1 ? "minute" : "minutes"} ago`;
    }
    if (timeDifference < 86400) {
        const hours = Math.floor(timeDifference / 3600);
        return `${hours} ${hours === 1 ? "hour" : "hours"} ago`;
    }
    const days = Math.floor(timeDifference / 86400);
    return `${days} ${days === 1 ? "day" : "days"} ago`;
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

    table_empty_display()
    set_table_name_row_onclick()
    choose_icon()
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

function choose_icon() {
    let table = document.getElementById('table');
    let rows = table.getElementsByTagName('tr');

    for (let i = 1; i < rows.length; i++) { // starting from 1 to skip the header row
        let firstTd = rows[i].getElementsByTagName('td')[0];
        let firstDiv = firstTd.getElementsByTagName('div')[0];

        let name = firstTd.innerText.toLowerCase();
        let isDir = name.endsWith('/');
        let positionText;

        if (isDir) {
            positionText = "-128px 0px";
        } else {
            let extension = name.split('.').pop();

            switch(extension) {
                case 'rs':
                    positionText = "0px -128px";
                    break;
                case 'iso':
                    positionText = "-384px 0px";
                    break;
                case 'json':
                case 'js':
                    positionText = "-512px 0px";
                    break;
                case 'py':
                    positionText = "-640px 0px";
                    break;
                case 'zip':
                case 'gz':
                case 'rar':
                case '7z':
                case 'tar':
                case 'bz2':
                case 'xz':
                    positionText = "-768px 0px";
                    break;
                case 'pdf':
                    positionText = "-896px 0px";
                    break;
                case 'jpg':
                case 'jpeg':
                    positionText = "-512px -128px";
                    break;
                case 'svg':
                    positionText = "-384px -128px";
                    break;
                case 'png':
                    positionText = "-640px -128px";
                    break;
                case 'gif':
                    positionText = "-896px -128px";
                    break;
                case 'ds_store':
                    positionText = "-768px -128px";
                    break;
                default:
                    positionText = "-256px 0px";
            }
        }

        firstDiv.style.backgroundPosition = positionText;
    }
}


function navurl(url) {
    window.location.href = url;
}


function first_run_manifest() {
    formatTableRows()
}

document.addEventListener('DOMContentLoaded', first_run_manifest);
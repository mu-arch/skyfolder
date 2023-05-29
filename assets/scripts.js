let GLOBAL_table_data;

function navurl(url) {
    window.location.href = url;
}

/*
function levenshtein(a, b) {
    let t = [], u, i, j, m = a.length, n = b.length;
    if (!m) return n;
    if (!n) return m;
    for (j = 0; j <= n; j++) t[j] = j;
    for (i = 1; i <= m; i++) {
        for (u = [i], j = 1; j <= n; j++) {
            u[j] = a[i-1] === b[j-1] ? t[j-1] : Math.min(t[j-1], t[j], u[j-1]) + 1;
        }
        t = u;
    }
    return t[n];
}
 */

function damerauLevenshtein(a, b) {
    let matrix = [];
    let aLen = a.length;
    let bLen = b.length;

    for (let i = 0; i <= aLen; i++) {
        matrix[i] = [];
        matrix[i][0] = i;
    }

    for (let j = 0; j <= bLen; j++) {
        matrix[0][j] = j;
    }

    for (let i = 1; i <= aLen; i++) {
        for (let j = 1; j <= bLen; j++) {
            let cost = a[i - 1] === b[j - 1] ? 0 : 1;

            matrix[i][j] = Math.min(
                matrix[i - 1][j] + 1, // deletion
                matrix[i][j - 1] + 1, // insertion
                matrix[i - 1][j - 1] + cost // substitution
            );

            if (i > 1 && j > 1 && a[i - 1] === b[j - 2] && a[i - 2] === b[j - 1]) {
                matrix[i][j] = Math.min(
                    matrix[i][j],
                    matrix[i - 2][j - 2] + cost // transposition
                );
            }
        }
    }

    return matrix[aLen][bLen];
}

function fuzzySearch(list, query, n) {
    let results = list.map((item) => ({ item, distance: damerauLevenshtein(item, query) }));
    results.sort((a, b) => a.distance - b.distance);
    return results.slice(0, n);
}

function checkIfTbodyIsEmpty() {
    const table = document.querySelector('#table');
    const tbody = table.querySelector('tbody');

    if (tbody.children.length === 0) {
        table.querySelector('thead tr:last-child').style = 'border-bottom: 1px solid #e5e5e5';
    }
}

function populateTable(tableBody, data) {
    // clear table
    while (tableBody.firstChild) {
        tableBody.removeChild(tableBody.firstChild);
    }

    // populate table with data
    data.forEach((row) => {
        let tr = document.createElement('tr');
        let tdName = document.createElement('td');
        let tdModified = document.createElement('td');
        let tdSize = document.createElement('td');

        let aName = document.createElement('a');
        aName.href = row.item.name;
        aName.textContent = row.item.name;
        tdName.appendChild(aName);

        tdModified.textContent = row.item.modified;
        tdSize.textContent = row.item.size;

        tr.appendChild(tdName);
        tr.appendChild(tdModified);
        tr.appendChild(tdSize);

        tableBody.appendChild(tr);
    });
}


function convertTableToObject(tableBody) {
    let data = [];
    let rows = tableBody.rows;
    for (let i = 0; i < rows.length; i++) {
        let row = rows[i];
        let cols = row.cells;
        let rowData = {
            name: cols[0].getElementsByTagName('a')[0].textContent,
            modified: cols[1].getAttribute('aria-modified'),
            size: cols[2].getAttribute('aria-size'),
        };
        data.push(rowData);
    }
    return data;
}

function searchAndPopulateTable(searchQuery, resultsLimit) {
    let searchResults = fuzzySearch(GLOBAL_table_data, searchQuery, resultsLimit);
    populateTable(tableBody, searchResults);
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
    checkIfTbodyIsEmpty()
}

document.addEventListener('DOMContentLoaded', first_run_manifest);
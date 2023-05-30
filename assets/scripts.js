'use strict';

let GLOBAL_TABLE_DATA;

function navurl(url) {
    window.location.href = url;
}

function check_if_tbody_is_empty() {
    const table = document.querySelector('#table');
    const tbody = table.querySelector('tbody');

    if (tbody.children.length === 0) {
        table.querySelector('thead tr:last-child').style = 'border-bottom: 1px solid #e5e5e5';
    }
}
function extractTableData() {
    const tbody = document.querySelector('tbody');  // get the tbody element
    const rows = tbody.querySelectorAll('tr'); // get all 'tr' elements within the tbody

    // Map each 'tr' to an array and return the result
    return Array.from(rows).map((row, index) => {
        const aTag = row.querySelector('a');  // get the 'a' tag within the current 'tr'
        return [aTag.innerText.toLowerCase(), index]; // return the array for the current 'tr', and we lowercase the inner text ready to search
    });
}

function fuzzysearch(needle, haystack) {
    var hlen = haystack.length;
    var nlen = needle.length;
    if (nlen > hlen) {
        return false;
    }
    if (nlen === hlen) {
        return needle === haystack;
    }
    outer: for (var i = 0, j = 0; i < nlen; i++) {
        var nch = needle.charCodeAt(i);
        while (j < hlen) {
            if (haystack.charCodeAt(j++) === nch) {
                continue outer;
            }
        }
        return false;
    }
    return true;
}
function marshall_search(query, rowMatrix) {
    const matches = rowMatrix
        .map((row, index) => ({
            index: row[1],  // index of the original row
            match: fuzzysearch(query, row[0])  // does query match the string?
        }))
        .filter(m => m.match)  // keep only rows where query matches the string

    // return the indexes of all matching rows
    return matches.map(m => m.index);
}

function displaySearchResults(indexes) {
    const tbody = document.getElementById('b');  // get the original tbody
    tbody.style.display = 'none';  // set the original tbody to display none

    // if b2 already exists, delete it
    const oldB2 = document.getElementById('b2');
    if (oldB2) {
        oldB2.remove();
    }

    // create a new tbody with id="b2"
    const b2 = document.createElement('tbody');
    b2.id = 'b2';

    // for each index, copy the corresponding row from the original tbody to b2
    for (const index of indexes) {
        const row = tbody.rows[index];
        const clonedRow = row.cloneNode(true);  // clone the row
        b2.appendChild(clonedRow);  // append the cloned row to b2
    }

    // insert b2 after the original tbody
    tbody.parentNode.insertBefore(b2, tbody.nextSibling);
}


function search(query) {
    let results = marshall_search(query, GLOBAL_TABLE_DATA);
    displaySearchResults(results)
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
    check_if_tbody_is_empty()
    GLOBAL_TABLE_DATA = extractTableData();
}

document.addEventListener('DOMContentLoaded', first_run_manifest);
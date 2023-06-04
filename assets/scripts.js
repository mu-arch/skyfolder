'use strict';

let GLOBAL_TABLE_DATA;

function navurl(url) {
    window.location.href = url;
}

function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
};

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

function workerSearch() {
    self.onmessage = function(event) {
        const { query, rowMatrix } = event.data;

        function fuzzysearch(needle, haystack) {
            var hlen = haystack.length;
            var nlen = needle.length;
            if (nlen > hlen) {
                return false;
            }
            if (nlen === hlen) {
                return haystack.startsWith(needle);
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

        const matches = rowMatrix
            .map((row, index) => ({
                index: row[1],
                match: fuzzysearch(query, row[0]),
                // Save the relevance score
                relevance: row[0].includes(query) ? 0 : 1
            }))
            .filter(m => m.match !== false)
            .sort((a, b) => a.relevance - b.relevance) // Lower scores first
            .map(m => m.index);

        self.postMessage(matches);
    };
}

function marshall_search(query, rowMatrix) {
    return new Promise((resolve, reject) => {
        let workerCode = workerSearch.toString();
        workerCode = workerCode.slice(workerCode.indexOf("{") + 1, workerCode.lastIndexOf("}"));

        const workerBlob = new Blob([workerCode], { type: "text/javascript" });
        const workerURL = URL.createObjectURL(workerBlob);

        const worker = new Worker(workerURL);
        worker.onmessage = function(event) {
            resolve(event.data);
        };
        worker.onerror = function(error) {
            reject(error);
        };
        worker.postMessage({ query, rowMatrix });
    });
}






function displaySearchResults(indexes, searchTerm, limit = Infinity) {
    requestAnimationFrame(() => {
        const tbody = document.getElementById('b');
        tbody.style.display = 'none';

        const oldB2 = document.getElementById('b2');
        if (oldB2) {
            oldB2.remove();
        }

        const b2 = document.createElement('tbody');
        b2.id = 'b2';

        const limitedIndexes = indexes.slice(0, limit);

        const fragment = document.createDocumentFragment();

        for (const index of limitedIndexes) {
            const row = tbody.rows[index];
            const clonedRow = row.cloneNode(true);
            const aTag = clonedRow.querySelector('td:first-child a');
            if (aTag) {
                const textContent = aTag.innerHTML;
                const searchTermRegExp = new RegExp(`(${searchTerm})`, 'ig'); // 'i' makes it case-insensitive, 'g' is for global search
                const newTextContent = textContent.replace(searchTermRegExp, function(match) {
                    return `<span>${match}</span>`;
                });
                aTag.innerHTML = newTextContent;
            }
            fragment.appendChild(clonedRow);
        }

        b2.appendChild(fragment);

        // Add omitted results caption
        const omittedResults = indexes.length - limitedIndexes.length;
        const omittedRows = GLOBAL_TABLE_DATA.length - limitedIndexes.length;
        const resultsCaption = document.createElement('caption');
        resultsCaption.title = "Click to show all omitted matches.";
        if (omittedResults > 0) {
            resultsCaption.innerHTML = `<span><b>${omittedResults}</b> omitted matches</span> <div>-</div> <b>${omittedRows}</b> total rows excluded.`;
        } else {
            resultsCaption.innerHTML = `<b>${omittedResults}</b> omitted matches <div>-</div> <b>${omittedRows}</b> total rows excluded`;
        }

        resultsCaption.addEventListener('click', () => {
            search(searchTerm, Infinity)
        });

        b2.appendChild(resultsCaption);

        tbody.parentNode.insertBefore(b2, tbody.nextSibling);
    });
}


function cleanupSearchResults() {
    // get the original tbody
    const tbody = document.getElementById('b');
    // set the original tbody to display block (or '' to revert to default CSS value)
    tbody.style.display = '';

    // get b2
    const oldB2 = document.getElementById('b2');
    // if b2 exists, delete it
    if (oldB2) {
        oldB2.remove();
    }
}



function search(query, resultLimit) {
    marshall_search(query, GLOBAL_TABLE_DATA)
        .then(results => {
            displaySearchResults(results, query.toLowerCase(), resultLimit)
        });
}

var handleSearchInput = (function() {
    var debouncedSearch = debounce(search, 100);

    return function(event) {
        var val = event.target.value;
        if (val === "") {
            cleanupSearchResults();
        } else {
            if (GLOBAL_TABLE_DATA.length > 200) {
                debouncedSearch(val, 50);
            } else {
                search(val, 50);
            }
        }
    };
})();



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

//manifests run at pageload
function file_dir_manifest() {
    check_if_tbody_is_empty()
    GLOBAL_TABLE_DATA = extractTableData();
    document.querySelector('header input').addEventListener('input', handleSearchInput);

}

document.addEventListener('DOMContentLoaded', file_dir_manifest);
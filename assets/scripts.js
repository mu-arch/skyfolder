function navurl(url) {
    window.location.href = url;
}

function checkIfTbodyIsEmpty() {
    const table = document.querySelector('#table');
    const tbody = table.querySelector('tbody');

    if (tbody.children.length === 0) {
        table.querySelector('thead tr:last-child').style = 'border-bottom: 1px solid #e5e5e5';
    }
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
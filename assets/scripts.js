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

}

document.addEventListener('DOMContentLoaded', first_run_manifest);
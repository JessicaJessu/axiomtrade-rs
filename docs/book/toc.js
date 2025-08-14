// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded "><a href="installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="quick-start.html"><strong aria-hidden="true">2.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="environment-setup.html"><strong aria-hidden="true">3.</strong> Environment Setup</a></li><li class="chapter-item expanded "><a href="automatic-otp.html"><strong aria-hidden="true">4.</strong> Automatic OTP</a></li><li class="chapter-item expanded "><a href="auth/login.html"><strong aria-hidden="true">5.</strong> Login &amp; Sessions</a></li><li class="chapter-item expanded "><a href="auth/tokens.html"><strong aria-hidden="true">6.</strong> Token Management</a></li><li class="chapter-item expanded "><a href="auth/sessions.html"><strong aria-hidden="true">7.</strong> Session Management</a></li><li class="chapter-item expanded "><a href="auth/otp.html"><strong aria-hidden="true">8.</strong> OTP Verification</a></li><li class="chapter-item expanded "><a href="api/portfolio.html"><strong aria-hidden="true">9.</strong> Portfolio Management</a></li><li class="chapter-item expanded "><a href="api/trading.html"><strong aria-hidden="true">10.</strong> Trading Operations</a></li><li class="chapter-item expanded "><a href="api/market-data.html"><strong aria-hidden="true">11.</strong> Market Data</a></li><li class="chapter-item expanded "><a href="api/websocket.html"><strong aria-hidden="true">12.</strong> WebSocket Streaming</a></li><li class="chapter-item expanded "><a href="api/notifications.html"><strong aria-hidden="true">13.</strong> Notifications</a></li><li class="chapter-item expanded "><a href="api/turnkey.html"><strong aria-hidden="true">14.</strong> Turnkey Integration</a></li><li class="chapter-item expanded "><a href="examples/authentication.html"><strong aria-hidden="true">15.</strong> Authentication Examples</a></li><li class="chapter-item expanded "><a href="examples/portfolio.html"><strong aria-hidden="true">16.</strong> Portfolio Examples</a></li><li class="chapter-item expanded "><a href="examples/trading.html"><strong aria-hidden="true">17.</strong> Trading Examples</a></li><li class="chapter-item expanded "><a href="examples/websocket.html"><strong aria-hidden="true">18.</strong> WebSocket Examples</a></li><li class="chapter-item expanded "><a href="examples/advanced.html"><strong aria-hidden="true">19.</strong> Advanced Examples</a></li><li class="chapter-item expanded "><a href="best-practices/error-handling.html"><strong aria-hidden="true">20.</strong> Error Handling</a></li><li class="chapter-item expanded "><a href="best-practices/rate-limiting.html"><strong aria-hidden="true">21.</strong> Rate Limiting</a></li><li class="chapter-item expanded "><a href="best-practices/security.html"><strong aria-hidden="true">22.</strong> Security</a></li><li class="chapter-item expanded "><a href="best-practices/performance.html"><strong aria-hidden="true">23.</strong> Performance</a></li><li class="chapter-item expanded "><a href="troubleshooting/common-issues.html"><strong aria-hidden="true">24.</strong> Common Issues</a></li><li class="chapter-item expanded "><a href="troubleshooting/debugging.html"><strong aria-hidden="true">25.</strong> Debug Guide</a></li><li class="chapter-item expanded "><a href="troubleshooting/faq.html"><strong aria-hidden="true">26.</strong> FAQ</a></li><li class="chapter-item expanded "><a href="reference/configuration.html"><strong aria-hidden="true">27.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="reference/error-codes.html"><strong aria-hidden="true">28.</strong> Error Codes</a></li><li class="chapter-item expanded "><a href="reference/endpoints.html"><strong aria-hidden="true">29.</strong> API Endpoints</a></li><li class="chapter-item expanded "><a href="reference/changelog.html"><strong aria-hidden="true">30.</strong> Changelog</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);

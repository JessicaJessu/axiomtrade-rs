// axiomtrade-rs Documentation JavaScript
// Professional enhancements for documentation site

(function() {
    'use strict';

    // Wait for DOM to be ready
    document.addEventListener('DOMContentLoaded', function() {
        initializeDocumentation();
    });

    function initializeDocumentation() {
        addCopyButtons();
        enhanceNavigation();
        addTableOfContents();
        improveCodeBlocks();
        addVersionBanner();
    }

    // Add copy buttons to code blocks
    function addCopyButtons() {
        const codeBlocks = document.querySelectorAll('pre code');
        
        codeBlocks.forEach(function(codeBlock) {
            const pre = codeBlock.parentNode;
            const button = document.createElement('button');
            
            button.className = 'copy-button';
            button.textContent = 'Copy';
            button.setAttribute('aria-label', 'Copy code to clipboard');
            
            button.addEventListener('click', function() {
                copyToClipboard(codeBlock.textContent, button);
            });
            
            pre.style.position = 'relative';
            pre.appendChild(button);
        });
    }

    // Copy text to clipboard
    function copyToClipboard(text, button) {
        navigator.clipboard.writeText(text).then(function() {
            button.textContent = 'Copied!';
            button.style.backgroundColor = '#28A745';
            
            setTimeout(function() {
                button.textContent = 'Copy';
                button.style.backgroundColor = '';
            }, 2000);
        }).catch(function() {
            // Fallback for older browsers
            const textArea = document.createElement('textarea');
            textArea.value = text;
            document.body.appendChild(textArea);
            textArea.select();
            document.execCommand('copy');
            document.body.removeChild(textArea);
            
            button.textContent = 'Copied!';
            setTimeout(function() {
                button.textContent = 'Copy';
            }, 2000);
        });
    }

    // Enhance navigation with smooth scrolling
    function enhanceNavigation() {
        const navLinks = document.querySelectorAll('a[href^="#"]');
        
        navLinks.forEach(function(link) {
            link.addEventListener('click', function(e) {
                const targetId = this.getAttribute('href').substring(1);
                const target = document.getElementById(targetId);
                
                if (target) {
                    e.preventDefault();
                    target.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                }
            });
        });
    }

    // Add table of contents for long pages
    function addTableOfContents() {
        const content = document.querySelector('.content');
        const headings = content.querySelectorAll('h2, h3, h4');
        
        if (headings.length > 3) {
            const toc = createTableOfContents(headings);
            const firstH2 = content.querySelector('h2');
            
            if (firstH2) {
                firstH2.parentNode.insertBefore(toc, firstH2);
            }
        }
    }

    // Create table of contents element
    function createTableOfContents(headings) {
        const tocContainer = document.createElement('div');
        tocContainer.className = 'table-of-contents';
        
        const tocTitle = document.createElement('h3');
        tocTitle.textContent = 'Table of Contents';
        tocContainer.appendChild(tocTitle);
        
        const tocList = document.createElement('ul');
        
        headings.forEach(function(heading, index) {
            // Add ID if it doesn't exist
            if (!heading.id) {
                heading.id = 'heading-' + index;
            }
            
            const listItem = document.createElement('li');
            const link = document.createElement('a');
            
            link.href = '#' + heading.id;
            link.textContent = heading.textContent;
            link.className = 'toc-' + heading.tagName.toLowerCase();
            
            listItem.appendChild(link);
            tocList.appendChild(listItem);
        });
        
        tocContainer.appendChild(tocList);
        return tocContainer;
    }

    // Improve code blocks with language labels
    function improveCodeBlocks() {
        const codeBlocks = document.querySelectorAll('pre code[class*="language-"]');
        
        codeBlocks.forEach(function(codeBlock) {
            const className = codeBlock.className;
            const language = className.match(/language-(\w+)/);
            
            if (language) {
                const pre = codeBlock.parentNode;
                const label = document.createElement('div');
                
                label.className = 'code-language-label';
                label.textContent = language[1].toUpperCase();
                
                pre.insertBefore(label, codeBlock);
            }
        });
    }

    // Add version banner if this is a development version
    function addVersionBanner() {
        const isLocalhost = window.location.hostname === 'localhost' || 
                           window.location.hostname === '127.0.0.1';
        
        if (isLocalhost) {
            const banner = document.createElement('div');
            banner.className = 'dev-banner';
            banner.innerHTML = '<strong>Development Version</strong> - You are viewing the local development documentation';
            
            document.body.insertBefore(banner, document.body.firstChild);
        }
    }

    // Add search enhancement
    function enhanceSearch() {
        const searchInput = document.querySelector('#searchbar');
        
        if (searchInput) {
            searchInput.setAttribute('placeholder', 'Search documentation...');
            
            // Add keyboard shortcut (Ctrl+K or Cmd+K)
            document.addEventListener('keydown', function(e) {
                if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                    e.preventDefault();
                    searchInput.focus();
                }
            });
        }
    }

    // Initialize search enhancement
    enhanceSearch();

})();

// Add CSS for JavaScript-added elements
const style = document.createElement('style');
style.textContent = `
    .copy-button {
        position: absolute;
        top: 0.5rem;
        right: 0.5rem;
        background-color: #333;
        color: white;
        border: none;
        padding: 0.3rem 0.6rem;
        border-radius: 3px;
        font-size: 0.8rem;
        cursor: pointer;
        opacity: 0.7;
        transition: opacity 0.2s ease;
    }

    .copy-button:hover {
        opacity: 1;
    }

    .table-of-contents {
        background-color: #F7F7F7;
        border: 1px solid #E4E4E4;
        border-radius: 4px;
        padding: 1rem;
        margin: 2rem 0;
    }

    .table-of-contents h3 {
        margin-top: 0;
        color: #333;
    }

    .table-of-contents ul {
        list-style: none;
        padding-left: 0;
    }

    .table-of-contents li {
        margin-bottom: 0.3rem;
    }

    .table-of-contents a {
        text-decoration: none;
        color: #E95420;
        font-size: 0.9rem;
    }

    .table-of-contents .toc-h3 {
        padding-left: 1rem;
        font-size: 0.85rem;
    }

    .table-of-contents .toc-h4 {
        padding-left: 2rem;
        font-size: 0.8rem;
    }

    .code-language-label {
        background-color: #333;
        color: white;
        font-size: 0.7rem;
        padding: 0.2rem 0.5rem;
        position: absolute;
        top: 0;
        right: 0;
        border-radius: 0 4px 0 4px;
        font-family: monospace;
    }

    .dev-banner {
        background-color: #FFF3CD;
        border-bottom: 1px solid #FFEAA7;
        padding: 0.5rem;
        text-align: center;
        font-size: 0.9rem;
        color: #856404;
    }

    pre {
        position: relative;
    }
`;
document.head.appendChild(style);
/* Comparison table between static versus agile development costs */
"use strict"
{
    class StaticVsAgileCompTable extends HTMLElement {
        constructor() {
            super();
            const t = this;
            if (t.parentNode && t.parentNode.nodeName === 'P') {
                let p = t.parentNode;
                t.parentNode.parentNode.insertBefore(t, t.parentNode);
                if (p.innerHTML.trim() === '') {
                    p.remove();
                }
            }
        }
        static get observedAttributes() {
            return [];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            if (oldValue === newValue) return;
            if (newValue === null || newValue === undefined) {
                delete this[property];
            } else {
                this[property] = newValue;
            }
        }
        connectedCallback() {
            this.innerHTML = "Comparison report coming soon!"
        }
        disconnectedCallback() { }
    }
    customElements.define('app-static-vs-agile-comp-table', StaticVsAgileCompTable);
}
!function(e){var t={};function i(o){if(t[o])return t[o].exports;var s=t[o]={i:o,l:!1,exports:{}};return e[o].call(s.exports,s,s.exports,i),s.l=!0,s.exports}i.m=e,i.c=t,i.d=function(e,t,o){i.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:o})},i.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},i.t=function(e,t){if(1&t&&(e=i(e)),8&t)return e;if(4&t&&"object"==typeof e&&e&&e.__esModule)return e;var o=Object.create(null);if(i.r(o),Object.defineProperty(o,"default",{enumerable:!0,value:e}),2&t&&"string"!=typeof e)for(var s in e)i.d(o,s,function(t){return e[t]}.bind(null,s));return o},i.n=function(e){var t=e&&e.__esModule?function(){return e.default}:function(){return e};return i.d(t,"a",t),t},i.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},i.p="",i(i.s=0)}([function(e,t,i){"use strict";i.r(t);
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
const o=new WeakMap,s=e=>"function"==typeof e&&o.has(e),n=void 0!==window.customElements&&void 0!==window.customElements.polyfillWrapFlushCallback,r=(e,t,i=null)=>{for(;t!==i;){const i=t.nextSibling;e.removeChild(t),t=i}},a={},l={},d=`{{lit-${String(Math.random()).slice(2)}}}`,c=`\x3c!--${d}--\x3e`,p=new RegExp(`${d}|${c}`),h="$lit$";class u{constructor(e,t){this.parts=[],this.element=t;const i=[],o=[],s=document.createTreeWalker(t.content,133,null,!1);let n=0,r=-1,a=0;const{strings:l,values:{length:c}}=e;for(;a<c;){const e=s.nextNode();if(null!==e){if(r++,1===e.nodeType){if(e.hasAttributes()){const t=e.attributes,{length:i}=t;let o=0;for(let e=0;e<i;e++)f(t[e].name,h)&&o++;for(;o-- >0;){const t=l[a],i=y.exec(t)[2],o=i.toLowerCase()+h,s=e.getAttribute(o);e.removeAttribute(o);const n=s.split(p);this.parts.push({type:"attribute",index:r,name:i,strings:n}),a+=n.length-1}}"TEMPLATE"===e.tagName&&(o.push(e),s.currentNode=e.content)}else if(3===e.nodeType){const t=e.data;if(t.indexOf(d)>=0){const o=e.parentNode,s=t.split(p),n=s.length-1;for(let t=0;t<n;t++){let i,n=s[t];if(""===n)i=m();else{const e=y.exec(n);null!==e&&f(e[2],h)&&(n=n.slice(0,e.index)+e[1]+e[2].slice(0,-h.length)+e[3]),i=document.createTextNode(n)}o.insertBefore(i,e),this.parts.push({type:"node",index:++r})}""===s[n]?(o.insertBefore(m(),e),i.push(e)):e.data=s[n],a+=n}}else if(8===e.nodeType)if(e.data===d){const t=e.parentNode;null!==e.previousSibling&&r!==n||(r++,t.insertBefore(m(),e)),n=r,this.parts.push({type:"node",index:r}),null===e.nextSibling?e.data="":(i.push(e),r--),a++}else{let t=-1;for(;-1!==(t=e.data.indexOf(d,t+1));)this.parts.push({type:"node",index:-1}),a++}}else s.currentNode=o.pop()}for(const e of i)e.parentNode.removeChild(e)}}const f=(e,t)=>{const i=e.length-t.length;return i>=0&&e.slice(i)===t},g=e=>-1!==e.index,m=()=>document.createComment(""),y=/([ \x09\x0a\x0c\x0d])([^\0-\x1F\x7F-\x9F "'>=/]+)([ \x09\x0a\x0c\x0d]*=[ \x09\x0a\x0c\x0d]*(?:[^ \x09\x0a\x0c\x0d"'`<>=]*|"[^"]*|'[^']*))$/;
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
class b{constructor(e,t,i){this.__parts=[],this.template=e,this.processor=t,this.options=i}update(e){let t=0;for(const i of this.__parts)void 0!==i&&i.setValue(e[t]),t++;for(const e of this.__parts)void 0!==e&&e.commit()}_clone(){const e=n?this.template.element.content.cloneNode(!0):document.importNode(this.template.element.content,!0),t=[],i=this.template.parts,o=document.createTreeWalker(e,133,null,!1);let s,r=0,a=0,l=o.nextNode();for(;r<i.length;)if(s=i[r],g(s)){for(;a<s.index;)a++,"TEMPLATE"===l.nodeName&&(t.push(l),o.currentNode=l.content),null===(l=o.nextNode())&&(o.currentNode=t.pop(),l=o.nextNode());if("node"===s.type){const e=this.processor.handleTextExpression(this.options);e.insertAfterNode(l.previousSibling),this.__parts.push(e)}else this.__parts.push(...this.processor.handleAttributeExpressions(l,s.name,s.strings,this.options));r++}else this.__parts.push(void 0),r++;return n&&(document.adoptNode(e),customElements.upgrade(e)),e}}
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */const v=` ${d} `;class _{constructor(e,t,i,o){this.strings=e,this.values=t,this.type=i,this.processor=o}getHTML(){const e=this.strings.length-1;let t="",i=!1;for(let o=0;o<e;o++){const e=this.strings[o],s=e.lastIndexOf("\x3c!--");i=(s>-1||i)&&-1===e.indexOf("--\x3e",s+1);const n=y.exec(e);t+=null===n?e+(i?v:c):e.substr(0,n.index)+n[1]+n[2]+h+n[3]+d}return t+=this.strings[e],t}getTemplateElement(){const e=document.createElement("template");return e.innerHTML=this.getHTML(),e}}
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
const x=e=>null===e||!("object"==typeof e||"function"==typeof e),w=e=>Array.isArray(e)||!(!e||!e[Symbol.iterator]);class S{constructor(e,t,i){this.dirty=!0,this.element=e,this.name=t,this.strings=i,this.parts=[];for(let e=0;e<i.length-1;e++)this.parts[e]=this._createPart()}_createPart(){return new P(this)}_getValue(){const e=this.strings,t=e.length-1;let i="";for(let o=0;o<t;o++){i+=e[o];const t=this.parts[o];if(void 0!==t){const e=t.value;if(x(e)||!w(e))i+="string"==typeof e?e:String(e);else for(const t of e)i+="string"==typeof t?t:String(t)}}return i+=e[t],i}commit(){this.dirty&&(this.dirty=!1,this.element.setAttribute(this.name,this._getValue()))}}class P{constructor(e){this.value=void 0,this.committer=e}setValue(e){e===a||x(e)&&e===this.value||(this.value=e,s(e)||(this.committer.dirty=!0))}commit(){for(;s(this.value);){const e=this.value;this.value=a,e(this)}this.value!==a&&this.committer.commit()}}class C{constructor(e){this.value=void 0,this.__pendingValue=void 0,this.options=e}appendInto(e){this.startNode=e.appendChild(m()),this.endNode=e.appendChild(m())}insertAfterNode(e){this.startNode=e,this.endNode=e.nextSibling}appendIntoPart(e){e.__insert(this.startNode=m()),e.__insert(this.endNode=m())}insertAfterPart(e){e.__insert(this.startNode=m()),this.endNode=e.endNode,e.endNode=this.startNode}setValue(e){this.__pendingValue=e}commit(){for(;s(this.__pendingValue);){const e=this.__pendingValue;this.__pendingValue=a,e(this)}const e=this.__pendingValue;e!==a&&(x(e)?e!==this.value&&this.__commitText(e):e instanceof _?this.__commitTemplateResult(e):e instanceof Node?this.__commitNode(e):w(e)?this.__commitIterable(e):e===l?(this.value=l,this.clear()):this.__commitText(e))}__insert(e){this.endNode.parentNode.insertBefore(e,this.endNode)}__commitNode(e){this.value!==e&&(this.clear(),this.__insert(e),this.value=e)}__commitText(e){const t=this.startNode.nextSibling,i="string"==typeof(e=null==e?"":e)?e:String(e);t===this.endNode.previousSibling&&3===t.nodeType?t.data=i:this.__commitNode(document.createTextNode(i)),this.value=e}__commitTemplateResult(e){const t=this.options.templateFactory(e);if(this.value instanceof b&&this.value.template===t)this.value.update(e.values);else{const i=new b(t,e.processor,this.options),o=i._clone();i.update(e.values),this.__commitNode(o),this.value=i}}__commitIterable(e){Array.isArray(this.value)||(this.value=[],this.clear());const t=this.value;let i,o=0;for(const s of e)i=t[o],void 0===i&&(i=new C(this.options),t.push(i),0===o?i.appendIntoPart(this):i.insertAfterPart(t[o-1])),i.setValue(s),i.commit(),o++;o<t.length&&(t.length=o,this.clear(i&&i.endNode))}clear(e=this.startNode){r(this.startNode.parentNode,e.nextSibling,this.endNode)}}class N{constructor(e,t,i){if(this.value=void 0,this.__pendingValue=void 0,2!==i.length||""!==i[0]||""!==i[1])throw new Error("Boolean attributes can only contain a single expression");this.element=e,this.name=t,this.strings=i}setValue(e){this.__pendingValue=e}commit(){for(;s(this.__pendingValue);){const e=this.__pendingValue;this.__pendingValue=a,e(this)}if(this.__pendingValue===a)return;const e=!!this.__pendingValue;this.value!==e&&(e?this.element.setAttribute(this.name,""):this.element.removeAttribute(this.name),this.value=e),this.__pendingValue=a}}class E extends S{constructor(e,t,i){super(e,t,i),this.single=2===i.length&&""===i[0]&&""===i[1]}_createPart(){return new k(this)}_getValue(){return this.single?this.parts[0].value:super._getValue()}commit(){this.dirty&&(this.dirty=!1,this.element[this.name]=this._getValue())}}class k extends P{}let A=!1;try{const e={get capture(){return A=!0,!1}};window.addEventListener("test",e,e),window.removeEventListener("test",e,e)}catch(e){}class T{constructor(e,t,i){this.value=void 0,this.__pendingValue=void 0,this.element=e,this.eventName=t,this.eventContext=i,this.__boundHandleEvent=e=>this.handleEvent(e)}setValue(e){this.__pendingValue=e}commit(){for(;s(this.__pendingValue);){const e=this.__pendingValue;this.__pendingValue=a,e(this)}if(this.__pendingValue===a)return;const e=this.__pendingValue,t=this.value,i=null==e||null!=t&&(e.capture!==t.capture||e.once!==t.once||e.passive!==t.passive),o=null!=e&&(null==t||i);i&&this.element.removeEventListener(this.eventName,this.__boundHandleEvent,this.__options),o&&(this.__options=O(e),this.element.addEventListener(this.eventName,this.__boundHandleEvent,this.__options)),this.value=e,this.__pendingValue=a}handleEvent(e){"function"==typeof this.value?this.value.call(this.eventContext||this.element,e):this.value.handleEvent(e)}}const O=e=>e&&(A?{capture:e.capture,passive:e.passive,once:e.once}:e.capture);
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */const R=new class{handleAttributeExpressions(e,t,i,o){const s=t[0];if("."===s){return new E(e,t.slice(1),i).parts}return"@"===s?[new T(e,t.slice(1),o.eventContext)]:"?"===s?[new N(e,t.slice(1),i)]:new S(e,t,i).parts}handleTextExpression(e){return new C(e)}};
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */function $(e){let t=j.get(e.type);void 0===t&&(t={stringsArray:new WeakMap,keyString:new Map},j.set(e.type,t));let i=t.stringsArray.get(e.strings);if(void 0!==i)return i;const o=e.strings.join(d);return i=t.keyString.get(o),void 0===i&&(i=new u(e,e.getTemplateElement()),t.keyString.set(o,i)),t.stringsArray.set(e.strings,i),i}const j=new Map,V=new WeakMap;
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
(window.litHtmlVersions||(window.litHtmlVersions=[])).push("1.1.2");const D=(e,...t)=>new _(e,t,"html",R),z=133;function M(e,t){const{element:{content:i},parts:o}=e,s=document.createTreeWalker(i,z,null,!1);let n=B(o),r=o[n],a=-1,l=0;const d=[];let c=null;for(;s.nextNode();){a++;const e=s.currentNode;for(e.previousSibling===c&&(c=null),t.has(e)&&(d.push(e),null===c&&(c=e)),null!==c&&l++;void 0!==r&&r.index===a;)r.index=null!==c?-1:r.index-l,n=B(o,n),r=o[n]}d.forEach(e=>e.parentNode.removeChild(e))}const U=e=>{let t=11===e.nodeType?0:1;const i=document.createTreeWalker(e,z,null,!1);for(;i.nextNode();)t++;return t},B=(e,t=-1)=>{for(let i=t+1;i<e.length;i++){const t=e[i];if(g(t))return i}return-1};
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
const q=(e,t)=>`${e}--${t}`;let L=!0;void 0===window.ShadyCSS?L=!1:void 0===window.ShadyCSS.prepareTemplateDom&&(console.warn("Incompatible ShadyCSS version detected. Please update to at least @webcomponents/webcomponentsjs@2.0.2 and @webcomponents/shadycss@1.3.1."),L=!1);const F=e=>t=>{const i=q(t.type,e);let o=j.get(i);void 0===o&&(o={stringsArray:new WeakMap,keyString:new Map},j.set(i,o));let s=o.stringsArray.get(t.strings);if(void 0!==s)return s;const n=t.strings.join(d);if(s=o.keyString.get(n),void 0===s){const i=t.getTemplateElement();L&&window.ShadyCSS.prepareTemplateDom(i,e),s=new u(t,i),o.keyString.set(n,s)}return o.stringsArray.set(t.strings,s),s},I=["html","svg"],W=new Set,H=(e,t,i)=>{W.add(e);const o=i?i.element:document.createElement("template"),s=t.querySelectorAll("style"),{length:n}=s;if(0===n)return void window.ShadyCSS.prepareTemplateStyles(o,e);const r=document.createElement("style");for(let e=0;e<n;e++){const t=s[e];t.parentNode.removeChild(t),r.textContent+=t.textContent}(e=>{I.forEach(t=>{const i=j.get(q(t,e));void 0!==i&&i.keyString.forEach(e=>{const{element:{content:t}}=e,i=new Set;Array.from(t.querySelectorAll("style")).forEach(e=>{i.add(e)}),M(e,i)})})})(e);const a=o.content;i?function(e,t,i=null){const{element:{content:o},parts:s}=e;if(null==i)return void o.appendChild(t);const n=document.createTreeWalker(o,z,null,!1);let r=B(s),a=0,l=-1;for(;n.nextNode();){for(l++,n.currentNode===i&&(a=U(t),i.parentNode.insertBefore(t,i));-1!==r&&s[r].index===l;){if(a>0){for(;-1!==r;)s[r].index+=a,r=B(s,r);return}r=B(s,r)}}}(i,r,a.firstChild):a.insertBefore(r,a.firstChild),window.ShadyCSS.prepareTemplateStyles(o,e);const l=a.querySelector("style");if(window.ShadyCSS.nativeShadow&&null!==l)t.insertBefore(l.cloneNode(!0),t.firstChild);else if(i){a.insertBefore(r,a.firstChild);const e=new Set;e.add(r),M(i,e)}};window.JSCompiler_renameProperty=(e,t)=>e;const J={toAttribute(e,t){switch(t){case Boolean:return e?"":null;case Object:case Array:return null==e?e:JSON.stringify(e)}return e},fromAttribute(e,t){switch(t){case Boolean:return null!==e;case Number:return null===e?null:Number(e);case Object:case Array:return JSON.parse(e)}return e}},G=(e,t)=>t!==e&&(t==t||e==e),K={attribute:!0,type:String,converter:J,reflect:!1,hasChanged:G},Y=Promise.resolve(!0),Q=1,X=4,Z=8,ee=16,te=32,ie="finalized";class oe extends HTMLElement{constructor(){super(),this._updateState=0,this._instanceProperties=void 0,this._updatePromise=Y,this._hasConnectedResolver=void 0,this._changedProperties=new Map,this._reflectingProperties=void 0,this.initialize()}static get observedAttributes(){this.finalize();const e=[];return this._classProperties.forEach((t,i)=>{const o=this._attributeNameForProperty(i,t);void 0!==o&&(this._attributeToPropertyMap.set(o,i),e.push(o))}),e}static _ensureClassProperties(){if(!this.hasOwnProperty(JSCompiler_renameProperty("_classProperties",this))){this._classProperties=new Map;const e=Object.getPrototypeOf(this)._classProperties;void 0!==e&&e.forEach((e,t)=>this._classProperties.set(t,e))}}static createProperty(e,t=K){if(this._ensureClassProperties(),this._classProperties.set(e,t),t.noAccessor||this.prototype.hasOwnProperty(e))return;const i="symbol"==typeof e?Symbol():`__${e}`;Object.defineProperty(this.prototype,e,{get(){return this[i]},set(t){const o=this[e];this[i]=t,this._requestUpdate(e,o)},configurable:!0,enumerable:!0})}static finalize(){const e=Object.getPrototypeOf(this);if(e.hasOwnProperty(ie)||e.finalize(),this[ie]=!0,this._ensureClassProperties(),this._attributeToPropertyMap=new Map,this.hasOwnProperty(JSCompiler_renameProperty("properties",this))){const e=this.properties,t=[...Object.getOwnPropertyNames(e),..."function"==typeof Object.getOwnPropertySymbols?Object.getOwnPropertySymbols(e):[]];for(const i of t)this.createProperty(i,e[i])}}static _attributeNameForProperty(e,t){const i=t.attribute;return!1===i?void 0:"string"==typeof i?i:"string"==typeof e?e.toLowerCase():void 0}static _valueHasChanged(e,t,i=G){return i(e,t)}static _propertyValueFromAttribute(e,t){const i=t.type,o=t.converter||J,s="function"==typeof o?o:o.fromAttribute;return s?s(e,i):e}static _propertyValueToAttribute(e,t){if(void 0===t.reflect)return;const i=t.type,o=t.converter;return(o&&o.toAttribute||J.toAttribute)(e,i)}initialize(){this._saveInstanceProperties(),this._requestUpdate()}_saveInstanceProperties(){this.constructor._classProperties.forEach((e,t)=>{if(this.hasOwnProperty(t)){const e=this[t];delete this[t],this._instanceProperties||(this._instanceProperties=new Map),this._instanceProperties.set(t,e)}})}_applyInstanceProperties(){this._instanceProperties.forEach((e,t)=>this[t]=e),this._instanceProperties=void 0}connectedCallback(){this._updateState=this._updateState|te,this._hasConnectedResolver&&(this._hasConnectedResolver(),this._hasConnectedResolver=void 0)}disconnectedCallback(){}attributeChangedCallback(e,t,i){t!==i&&this._attributeToProperty(e,i)}_propertyToAttribute(e,t,i=K){const o=this.constructor,s=o._attributeNameForProperty(e,i);if(void 0!==s){const e=o._propertyValueToAttribute(t,i);if(void 0===e)return;this._updateState=this._updateState|Z,null==e?this.removeAttribute(s):this.setAttribute(s,e),this._updateState=this._updateState&~Z}}_attributeToProperty(e,t){if(this._updateState&Z)return;const i=this.constructor,o=i._attributeToPropertyMap.get(e);if(void 0!==o){const e=i._classProperties.get(o)||K;this._updateState=this._updateState|ee,this[o]=i._propertyValueFromAttribute(t,e),this._updateState=this._updateState&~ee}}_requestUpdate(e,t){let i=!0;if(void 0!==e){const o=this.constructor,s=o._classProperties.get(e)||K;o._valueHasChanged(this[e],t,s.hasChanged)?(this._changedProperties.has(e)||this._changedProperties.set(e,t),!0!==s.reflect||this._updateState&ee||(void 0===this._reflectingProperties&&(this._reflectingProperties=new Map),this._reflectingProperties.set(e,s))):i=!1}!this._hasRequestedUpdate&&i&&this._enqueueUpdate()}requestUpdate(e,t){return this._requestUpdate(e,t),this.updateComplete}async _enqueueUpdate(){let e,t;this._updateState=this._updateState|X;const i=this._updatePromise;this._updatePromise=new Promise((i,o)=>{e=i,t=o});try{await i}catch(e){}this._hasConnected||await new Promise(e=>this._hasConnectedResolver=e);try{const e=this.performUpdate();null!=e&&await e}catch(e){t(e)}e(!this._hasRequestedUpdate)}get _hasConnected(){return this._updateState&te}get _hasRequestedUpdate(){return this._updateState&X}get hasUpdated(){return this._updateState&Q}performUpdate(){this._instanceProperties&&this._applyInstanceProperties();let e=!1;const t=this._changedProperties;try{e=this.shouldUpdate(t),e&&this.update(t)}catch(t){throw e=!1,t}finally{this._markUpdated()}e&&(this._updateState&Q||(this._updateState=this._updateState|Q,this.firstUpdated(t)),this.updated(t))}_markUpdated(){this._changedProperties=new Map,this._updateState=this._updateState&~X}get updateComplete(){return this._getUpdateComplete()}_getUpdateComplete(){return this._updatePromise}shouldUpdate(e){return!0}update(e){void 0!==this._reflectingProperties&&this._reflectingProperties.size>0&&(this._reflectingProperties.forEach((e,t)=>this._propertyToAttribute(t,this[t],e)),this._reflectingProperties=void 0)}updated(e){}firstUpdated(e){}}oe[ie]=!0;
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
const se=e=>t=>"function"==typeof t?((e,t)=>(window.customElements.define(e,t),t))(e,t):((e,t)=>{const{kind:i,elements:o}=t;return{kind:i,elements:o,finisher(t){window.customElements.define(e,t)}}})(e,t),ne=(e,t)=>"method"!==t.kind||!t.descriptor||"value"in t.descriptor?{kind:"field",key:Symbol(),placement:"own",descriptor:{},initializer(){"function"==typeof t.initializer&&(this[t.key]=t.initializer.call(this))},finisher(i){i.createProperty(t.key,e)}}:Object.assign({},t,{finisher(i){i.createProperty(t.key,e)}}),re=(e,t,i)=>{t.constructor.createProperty(i,e)};function ae(e){return(t,i)=>void 0!==i?re(e,t,i):ne(e,t)}const le="adoptedStyleSheets"in Document.prototype&&"replace"in CSSStyleSheet.prototype,de=Symbol();class ce{constructor(e,t){if(t!==de)throw new Error("CSSResult is not constructable. Use `unsafeCSS` or `css` instead.");this.cssText=e}get styleSheet(){return void 0===this._styleSheet&&(le?(this._styleSheet=new CSSStyleSheet,this._styleSheet.replaceSync(this.cssText)):this._styleSheet=null),this._styleSheet}toString(){return this.cssText}}const pe=(e,...t)=>{const i=t.reduce((t,i,o)=>t+(e=>{if(e instanceof ce)return e.cssText;if("number"==typeof e)return e;throw new Error(`Value passed to 'css' function must be a 'css' function result: ${e}. Use 'unsafeCSS' to pass non-literal values, but\n            take care to ensure page security.`)})(i)+e[o+1],e[0]);return new ce(i,de)};
/**
 * @license
 * Copyright (c) 2017 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
(window.litElementVersions||(window.litElementVersions=[])).push("2.2.1");const he=e=>e.flat?e.flat(1/0):function e(t,i=[]){for(let o=0,s=t.length;o<s;o++){const s=t[o];Array.isArray(s)?e(s,i):i.push(s)}return i}(e);class ue extends oe{static finalize(){super.finalize.call(this),this._styles=this.hasOwnProperty(JSCompiler_renameProperty("styles",this))?this._getUniqueStyles():this._styles||[]}static _getUniqueStyles(){const e=this.styles,t=[];if(Array.isArray(e)){he(e).reduceRight((e,t)=>(e.add(t),e),new Set).forEach(e=>t.unshift(e))}else e&&t.push(e);return t}initialize(){super.initialize(),this.renderRoot=this.createRenderRoot(),window.ShadowRoot&&this.renderRoot instanceof window.ShadowRoot&&this.adoptStyles()}createRenderRoot(){return this.attachShadow({mode:"open"})}adoptStyles(){const e=this.constructor._styles;0!==e.length&&(void 0===window.ShadyCSS||window.ShadyCSS.nativeShadow?le?this.renderRoot.adoptedStyleSheets=e.map(e=>e.styleSheet):this._needsShimAdoptedStyleSheets=!0:window.ShadyCSS.ScopingShim.prepareAdoptedCssText(e.map(e=>e.cssText),this.localName))}connectedCallback(){super.connectedCallback(),this.hasUpdated&&void 0!==window.ShadyCSS&&window.ShadyCSS.styleElement(this)}update(e){super.update(e);const t=this.render();t instanceof _&&this.constructor.render(t,this.renderRoot,{scopeName:this.localName,eventContext:this}),this._needsShimAdoptedStyleSheets&&(this._needsShimAdoptedStyleSheets=!1,this.constructor._styles.forEach(e=>{const t=document.createElement("style");t.textContent=e.cssText,this.renderRoot.appendChild(t)}))}render(){}}ue.finalized=!0,ue.render=(e,t,i)=>{if(!i||"object"!=typeof i||!i.scopeName)throw new Error("The `scopeName` option is required.");const o=i.scopeName,s=V.has(t),n=L&&11===t.nodeType&&!!t.host,a=n&&!W.has(o),l=a?document.createDocumentFragment():t;if(((e,t,i)=>{let o=V.get(t);void 0===o&&(r(t,t.firstChild),V.set(t,o=new C(Object.assign({templateFactory:$},i))),o.appendInto(t)),o.setValue(e),o.commit()})(e,l,Object.assign({templateFactory:F(o)},i)),a){const e=V.get(l);V.delete(l);const i=e.value instanceof b?e.value.template:void 0;H(o,l,i),r(t,t.firstChild),t.appendChild(l),V.set(t,e)}!s&&n&&window.ShadyCSS.styleElement(t.host)};var fe=pe`.todoapp {
    background: #fff;
    margin: 130px 0 40px 0;
    position: relative;
    box-shadow: 0 2px 4px 0 rgba(0, 0, 0, 0.2),
                0 25px 50px 0 rgba(0, 0, 0, 0.1);
}

.info {
    margin: 65px auto 0;
    color: #4d4d4d;
    font-size: 11px;
    text-shadow: 0 1px 0 rgba(255, 255, 255, 0.5);
    text-align: center;
}

.info p {
    line-height: 1;
}

.info a {
    color: inherit;
    text-decoration: none;
    font-weight: 400;
}

.info a:hover {
    text-decoration: underline;
}`,ge=function(e,t,i,o){var s,n=arguments.length,r=n<3?t:null===o?o=Object.getOwnPropertyDescriptor(t,i):o;if("object"==typeof Reflect&&"function"==typeof Reflect.decorate)r=Reflect.decorate(e,t,i,o);else for(var a=e.length-1;a>=0;a--)(s=e[a])&&(r=(n<3?s(r):n>3?s(t,i,r):s(t,i))||r);return n>3&&r&&Object.defineProperty(t,i,r),r};let me=class extends ue{render(){return D`
            <section class="todoapp">
                <slot></slot>
            </section>
            <footer class="info">
                <p><a href="https://github.com/dakom/todo-shipyard-lit-dominator"><u>Repo on Github</u></a></p>
            </footer>
        `}};me.styles=fe,me=ge([se("todo-app")],me);var ye=pe`:focus {
    outline: 0;
}

.hidden {
    display: none;
}

.new-todo,
.edit {
    position: relative;
    margin: 0;
    width: 100%;
    font-size: 24px;
    font-family: inherit;
    font-weight: inherit;
    line-height: 1.4em;
    color: inherit;
    padding: 6px;
    border: 1px solid #999;
    box-shadow: inset 0 -1px 5px 0 rgba(0, 0, 0, 0.2);
    box-sizing: border-box;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}

hr {
    margin: 20px 0;
    border: 0;
    border-top: 1px dashed #c5c5c5;
    border-bottom: 1px dashed #f7f7f7;
}

button {
    margin: 0;
    padding: 0;
    border: 0;
    background: none;
    font-size: 100%;
    vertical-align: baseline;
    font-family: inherit;
    font-weight: inherit;
    color: inherit;
    -webkit-appearance: none;
    appearance: none;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}`,be=pe`.footer {
    padding: 10px 15px;
    height: 20px;
    text-align: center;
    font-size: 15px;
    border-top: 1px solid #e6e6e6;
    color: #777;
}

.footer:before {
    content: '';
    position: absolute;
    right: 0;
    bottom: 0;
    left: 0;
    height: 50px;
    overflow: hidden;
    box-shadow: 0 1px 1px rgba(0, 0, 0, 0.2),
                0 8px 0 -3px #f6f6f6,
                0 9px 1px -3px rgba(0, 0, 0, 0.2),
                0 16px 0 -6px #f6f6f6,
                0 17px 2px -6px rgba(0, 0, 0, 0.2);
}

.todo-count {
    float: left;
    text-align: left;
}

.todo-count strong {
    font-weight: 300;
}

.filters {
    margin: 0;
    padding: 0;
    list-style: none;
    position: absolute;
    right: 0;
    left: 0;
}

.filters li {
    display: inline;
}

.filters li a {
    color: inherit;
    margin: 3px;
    padding: 3px 7px;
    text-decoration: none;
    border: 1px solid transparent;
    border-radius: 3px;
}

.filters li a:hover {
    border-color: rgba(175, 47, 47, 0.1);
}

.filters li a.selected {
    border-color: rgba(175, 47, 47, 0.2);
}


.clear-completed,
html .clear-completed:active {
    float: right;
    position: relative;
    line-height: 20px;
    text-decoration: none;
    cursor: pointer;
}

.clear-completed:hover {
    text-decoration: underline;
}

@media (max-width: 430px) {
    .footer {
        height: 50px;
    }

    .filters {
        bottom: 10px;
    }
}
`;
/**
 * @license
 * Copyright (c) 2018 The Polymer Project Authors. All rights reserved.
 * This code may only be used under the BSD style license found at
 * http://polymer.github.io/LICENSE.txt
 * The complete set of authors may be found at
 * http://polymer.github.io/AUTHORS.txt
 * The complete set of contributors may be found at
 * http://polymer.github.io/CONTRIBUTORS.txt
 * Code distributed by Google as part of the polymer project is also
 * subject to an additional IP rights grant found at
 * http://polymer.github.io/PATENTS.txt
 */
const ve=new WeakMap,_e=(xe=e=>t=>{if(!(t instanceof P)||t instanceof k||"class"!==t.committer.name||t.committer.parts.length>1)throw new Error("The `classMap` directive must be used in the `class` attribute and must be the only part in the attribute.");const{committer:i}=t,{element:o}=i;ve.has(t)||(o.className=i.strings.join(" "));const{classList:s}=o,n=ve.get(t);for(const t in n)t in e||s.remove(t);for(const t in e){const i=e[t];n&&i===n[t]||s[i?"add":"remove"](t)}ve.set(t,e)},(...e)=>{const t=xe(...e);return o.set(t,!0),t});var xe,we;!function(e){e[e.All=0]="All",e[e.Active=1]="Active",e[e.Completed=2]="Completed"}(we||(we={}));class Se extends CustomEvent{constructor(e){super("add-todo",{detail:e})}}class Pe extends CustomEvent{constructor(e){super("remove-todo",{detail:e})}}class Ce extends CustomEvent{constructor(e){super("toggle-todo",{detail:e})}}class Ne extends CustomEvent{constructor(e){super("change-todo",{detail:e})}}class Ee extends CustomEvent{constructor(e){super("toggle-all-todos",{detail:{complete:e}})}}class ke extends CustomEvent{constructor(){super("clear-completed")}}class Ae extends CustomEvent{constructor(e){super("reposition-list",{detail:e})}}var Te=function(e,t,i,o){var s,n=arguments.length,r=n<3?t:null===o?o=Object.getOwnPropertyDescriptor(t,i):o;if("object"==typeof Reflect&&"function"==typeof Reflect.decorate)r=Reflect.decorate(e,t,i,o);else for(var a=e.length-1;a>=0;a--)(s=e[a])&&(r=(n<3?s(r):n>3?s(t,i,r):s(t,i))||r);return n>3&&r&&Object.defineProperty(t,i,r),r};let Oe=class extends ue{constructor(){super(...arguments),this.total=0,this.remaining=0,this.completed=0,this.filter=we.All}render(){const{total:e,remaining:t,completed:i,filter:o}=this;return 0===e?D`${l}`:D`
                <footer class="footer">
                    <span class="todo-count">${$e(t)}</span>
                    <ul class="filters">
                        ${Re(we.All)(o)}
                        ${Re(we.Active)(o)}
                        ${Re(we.Completed)(o)}
                    </ul>
                    ${i?D`<button class="clear-completed" @click=${(()=>this.dispatchEvent(new ke)).bind(this)}>Clear completed</button>`:l}
                </footer>
            `}};Oe.styles=[ye,be],Te([ae({type:Number})],Oe.prototype,"total",void 0),Te([ae({type:Number})],Oe.prototype,"remaining",void 0),Te([ae({type:Number})],Oe.prototype,"completed",void 0),Te([ae({type:Number})],Oe.prototype,"filter",void 0),Oe=Te([se("todo-footer")],Oe);const Re=e=>t=>{const[i,o]={[we.All]:["","All"],[we.Active]:["active","Active"],[we.Completed]:["completed","Completed"]}[e],s=_e({selected:e===t});return D`
        <li><a href="#/${i}" class=${s}>${o}</a></li>
    `},$e=e=>D`${e} item${1!==e?"s":""} left`;var je=pe`h1 {
    position: absolute;
    top: -140px;
    width: 100%;
    font-size: 80px;
    font-weight: 200;
    text-align: center;
    color: #b83f45;
    -webkit-text-rendering: optimizeLegibility;
    -moz-text-rendering: optimizeLegibility;
    text-rendering: optimizeLegibility;
}`,Ve=function(e,t,i,o){var s,n=arguments.length,r=n<3?t:null===o?o=Object.getOwnPropertyDescriptor(t,i):o;if("object"==typeof Reflect&&"function"==typeof Reflect.decorate)r=Reflect.decorate(e,t,i,o);else for(var a=e.length-1;a>=0;a--)(s=e[a])&&(r=(n<3?s(r):n>3?s(t,i,r):s(t,i))||r);return n>3&&r&&Object.defineProperty(t,i,r),r};let De=class extends ue{render(){return D`
            <header class="header">
                <h1>todos</h1>
                <slot name="input"></slot>
            </header>
        `}};De.styles=je,De=Ve([se("todo-header")],De);var ze,Me=pe`input::-webkit-input-placeholder {
    font-style: italic;
    font-weight: 300;
    color: rgba(0, 0, 0, 0.4);
}

input::-moz-placeholder {
    font-style: italic;
    font-weight: 300;
    color: rgba(0, 0, 0, 0.4);
}

input::input-placeholder {
    font-style: italic;
    font-weight: 300;
    color: rgba(0, 0, 0, 0.4);
}

input.new-todo {
    padding: 16px 16px 16px 60px;
    border: none;
    background: rgba(0, 0, 0, 0.003);
    box-shadow: inset 0 -2px 1px rgba(0,0,0,0.03);
}`;!function(e){e[e.ENTER=13]="ENTER",e[e.ESC=27]="ESC"}(ze||(ze={}));var Ue=function(e,t,i,o){var s,n=arguments.length,r=n<3?t:null===o?o=Object.getOwnPropertyDescriptor(t,i):o;if("object"==typeof Reflect&&"function"==typeof Reflect.decorate)r=Reflect.decorate(e,t,i,o);else for(var a=e.length-1;a>=0;a--)(s=e[a])&&(r=(n<3?s(r):n>3?s(t,i,r):s(t,i))||r);return n>3&&r&&Object.defineProperty(t,i,r),r};let Be=class extends ue{check_keypress(e){if(e.keyCode===ze.ENTER){const t=e.target,i=t.value.trim();""!==i&&this.dispatchEvent(new Se({label:i})),t.value=""}}render(){return D`
            <input id="input-text" class="new-todo" @keydown=${this.check_keypress} placeholder="What needs to be done?" autofocus />
        `}};Be.styles=[ye,Me],Be=Ue([se("todo-input")],Be);var qe=pe`.todo-list {
    margin: 0;
    padding: 0;
    list-style: none;
}`,Le=function(e,t,i,o){var s,n=arguments.length,r=n<3?t:null===o?o=Object.getOwnPropertyDescriptor(t,i):o;if("object"==typeof Reflect&&"function"==typeof Reflect.decorate)r=Reflect.decorate(e,t,i,o);else for(var a=e.length-1;a>=0;a--)(s=e[a])&&(r=(n<3?s(r):n>3?s(t,i,r):s(t,i))||r);return n>3&&r&&Object.defineProperty(t,i,r),r};let Fe=class extends ue{render(){return D`
            <ul class="todo-list">
                <slot></slot>
            </ul>
        `}};Fe.styles=qe,Fe=Le([se("todo-list")],Fe);var Ie=pe`.main {
    position: relative;
    z-index: 2;
    border-top: 1px solid #e6e6e6;
}

.toggle-all {
    width: 1px;
    height: 1px;
    border: none; /* Mobile Safari */
    opacity: 0;
    position: absolute;
    right: 100%;
    bottom: 100%;
}

.toggle-all + label {
    width: 60px;
    height: 34px;
    font-size: 0;
    position: absolute;
    top: -52px;
    left: -13px;
    -webkit-transform: rotate(90deg);
    transform: rotate(90deg);
}

.toggle-all + label:before {
    content: '❯';
    font-size: 22px;
    color: #e6e6e6;
    padding: 10px 27px 10px 27px;
}

.toggle-all:checked + label:before {
    color: #737373;
}
`,We=function(e,t,i,o){var s,n=arguments.length,r=n<3?t:null===o?o=Object.getOwnPropertyDescriptor(t,i):o;if("object"==typeof Reflect&&"function"==typeof Reflect.decorate)r=Reflect.decorate(e,t,i,o);else for(var a=e.length-1;a>=0;a--)(s=e[a])&&(r=(n<3?s(r):n>3?s(t,i,r):s(t,i))||r);return n>3&&r&&Object.defineProperty(t,i,r),r};let He=class extends ue{constructor(){super(...arguments),this.len=0,this.all_completed=!1}render(){return D`
            ${0===this.len?l:D`
                    <section class="main">
                        <input id="toggle-all" class="toggle-all" type="checkbox" @change=${e=>this.dispatchEvent(new Ee(e.target.checked))} .checked=${this.all_completed} />
                        <label for="toggle-all">Mark all as complete</label>
                        <todo-list id="list">
                            <slot></slot>
                        </todo-list>
                    </section>
                `}
        `}};He.styles=Ie,We([ae({type:Number})],He.prototype,"len",void 0),We([ae({type:Boolean})],He.prototype,"all_completed",void 0),He=We([se("todo-main")],He);var Je,Ge=pe`li {
    position: relative;
    font-size: 24px;
    border-bottom: 1px solid #ededed;
}

li:last-child {
    border-bottom: none;
}

li.editing {
    border-bottom: none;
    padding: 0;
}

li.editing .edit {
    display: block;
    width: calc(100% - 43px);
    padding: 12px 16px;
    margin: 0 0 0 43px;
}

li.editing .view {
    display: none;
}

li .toggle {
    text-align: center;
    width: 40px;
    /* auto, since non-WebKit browsers doesn't support input styling */
    height: auto;
    position: absolute;
    top: 0;
    bottom: 0;
    left: 30px;
    margin: auto 0;
    border: none; /* Mobile Safari */
    -webkit-appearance: none;
    appearance: none;
}

li .toggle {
    opacity: 0;
}

li .toggle + label {
    margin-left: 30px;
    /*
        Firefox requires \`#\` to be escaped - https://bugzilla.mozilla.org/show_bug.cgi?id=922433
        IE and Edge requires *everything* to be escaped to render, so we do that instead of just the \`#\` - https://developer.microsoft.com/en-us/microsoft-edge/platform/issues/7157459/
    */
    background-image: url('data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23ededed%22%20stroke-width%3D%223%22/%3E%3C/svg%3E');
    background-repeat: no-repeat;
    background-position: center left;
}

li .toggle:checked + label {
    background-image: url('data:image/svg+xml;utf8,%3Csvg%20xmlns%3D%22http%3A//www.w3.org/2000/svg%22%20width%3D%2240%22%20height%3D%2240%22%20viewBox%3D%22-10%20-18%20100%20135%22%3E%3Ccircle%20cx%3D%2250%22%20cy%3D%2250%22%20r%3D%2250%22%20fill%3D%22none%22%20stroke%3D%22%23bddad5%22%20stroke-width%3D%223%22/%3E%3Cpath%20fill%3D%22%235dc2af%22%20d%3D%22M72%2025L42%2071%2027%2056l-4%204%2020%2020%2034-52z%22/%3E%3C/svg%3E');
}

li label {
    word-break: break-all;
    padding: 15px 15px 15px 60px;
    display: block;
    line-height: 1.2;
    transition: color 0.4s;
    font-weight: 400;
    color: #4d4d4d;
}

li.completed label {
    color: #cdcdcd;
    text-decoration: line-through;
}

li .destroy {
    display: none;
    position: absolute;
    top: 0;
    right: 10px;
    bottom: 0;
    width: 40px;
    height: 40px;
    margin: auto 0;
    font-size: 30px;
    color: #cc9a9a;
    margin-bottom: 11px;
    transition: color 0.2s ease-out;
}


li .destroy:hover {
    color: #af5b5e;
}

li .destroy:after {
    content: '×';
}

li:hover .destroy {
    display: block;
}

div .dragicon {
    display: none;
    position: absolute;
    left: 10px;
    height: 60px;
    line-height: 60px;
    color: grey;
    cursor: pointer;
}
div:hover .dragicon {
    display: block;
}

li .edit {
    display: none;
}

li.editing:last-child {
    margin-bottom: -1px;
}

li.hidden {
    visibility: hidden;
}

div.dropside-before {
    border-top: blue 1px solid;
}
div.dropside-after {
    border-bottom: blue 1px solid;
}`;!function(e){e[e.None=0]="None",e[e.Before=1]="Before",e[e.After=2]="After"}(Je||(Je={}));var Ke=function(e,t,i,o){var s,n=arguments.length,r=n<3?t:null===o?o=Object.getOwnPropertyDescriptor(t,i):o;if("object"==typeof Reflect&&"function"==typeof Reflect.decorate)r=Reflect.decorate(e,t,i,o);else for(var a=e.length-1;a>=0;a--)(s=e[a])&&(r=(n<3?s(r):n>3?s(t,i,r):s(t,i))||r);return n>3&&r&&Object.defineProperty(t,i,r),r};let Ye=class extends ue{constructor(){super(...arguments),this.item=null,this.editing=!1,this.draggable=!1,this.dragging=!1,this.dropside=Je.None}render(){const{label:e,id:t,complete:i}=this.item,o=e=>{this.dragging||(this.dropside=e.offsetY<25?Je.Before:Je.After,e.preventDefault())},s=o,n=o;return this.editing?D`<todo-edit-line @stop-editing=${()=>this.editing=!1} .on_change=${e=>this.dispatchEvent(new Ne({id:t,label:e}))} .label=${e} .item_id=${t} />`:D`
                <li class=${_e({completed:i,hidden:this.dragging})} 
                    .draggable=${this.draggable} 
                    @dragend=${(e=>{this.dragging=!1}).bind(this)} 
                    @dragover=${n.bind(this)}
                    @dragenter=${s.bind(this)}
                    @dragleave=${(e=>this.dropside=Je.None).bind(this)}
                    @dragstart=${(e=>{requestAnimationFrame(()=>this.dragging=!0),e.dataTransfer.setData("text/plain",JSON.stringify(t))}).bind(this)}
                    @drop=${(e=>{const i=JSON.parse(e.dataTransfer.getData("text/plain"));this.dropside!==Je.None&&this.dispatchEvent(new Ae({src:i,dest:t,side:this.dropside})),this.dropside=Je.None,e.preventDefault()}).bind(this)}
                    >
                    <div class="view" class=${_e({"dropside-before":this.dropside===Je.Before,"dropside-after":this.dropside===Je.After})}>
                        <div class="dragicon" @mouseover=${(()=>this.draggable=!0).bind(this)} @mouseout=${(()=>this.draggable=!1).bind(this)}>☰</div>
                        <input class="toggle" type="checkbox" .checked=${i} @change=${(e=>this.dispatchEvent(new Ce({id:t,complete:e.target.checked}))).bind(this)}/>
                        <label @dblclick=${()=>this.editing=!0}>${e}</label>
                        <button class="destroy" @click=${(()=>this.dispatchEvent(new Pe({id:t}))).bind(this)} ></button>
                    </div>
                </li>
            `}};Ye.styles=[ye,Ge],Ke([ae({type:Object})],Ye.prototype,"item",void 0),Ke([ae({type:Boolean})],Ye.prototype,"editing",void 0),Ke([ae({type:Boolean})],Ye.prototype,"draggable",void 0),Ke([ae({type:Boolean})],Ye.prototype,"dragging",void 0),Ke([ae({type:Number})],Ye.prototype,"dropside",void 0),Ye=Ke([se("todo-item")],Ye);let Qe=class extends ue{constructor(){super(...arguments),this.label="",this.item_id="",this.on_change=null}stop_editing(){this.dispatchEvent(new Event("stop-editing"))}firstUpdated(){this.shadowRoot.getElementById("input").focus()}render(){return D`
                <li class="editing" >
                    <input id="input" type="text" class="edit" .value=${this.label} 
                        @blur=${this.stop_editing}
                        @keydown=${e=>{const{keyCode:t}=e;if(t===ze.ENTER||t===ze.ESC){if(e.keyCode===ze.ENTER){const t=e.target,i=t.value.trim();""!==i&&this.on_change(i),t.value=""}this.stop_editing()}}} 
                    />
                </li>
            `}};Qe.styles=[ye,Ge],Ke([ae({type:String})],Qe.prototype,"label",void 0),Ke([ae({type:String})],Qe.prototype,"item_id",void 0),Ke([ae({type:Function})],Qe.prototype,"on_change",void 0),Qe=Ke([se("todo-edit-line")],Qe),window.load_wasm(e=>{e.init_app()})}]);
parcelRequire=function(e,r,n,t){var i="function"==typeof parcelRequire&&parcelRequire,o="function"==typeof require&&require;function u(n,t){if(!r[n]){if(!e[n]){var f="function"==typeof parcelRequire&&parcelRequire;if(!t&&f)return f(n,!0);if(i)return i(n,!0);if(o&&"string"==typeof n)return o(n);var c=new Error("Cannot find module '"+n+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[n][1][r]||r},p.cache={};var l=r[n]=new u.Module(n);e[n][0].call(l.exports,p,l,l.exports,this)}return r[n].exports;function p(e){return u(p.resolve(e))}}u.isParcelRequire=!0,u.Module=function(e){this.id=e,this.bundle=u,this.exports={}},u.modules=e,u.cache=r,u.parent=i,u.register=function(r,n){e[r]=[function(e,r){r.exports=n},{}]};for(var f=0;f<n.length;f++)u(n[f]);if(n.length){var c=u(n[n.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=c:"function"==typeof define&&define.amd?define(function(){return c}):t&&(this[t]=c)}return u}({"SgWw":[function(require,module,exports) {
"use strict";Object.defineProperty(exports,"__esModule",{value:!0}),exports.__wbindgen_throw=exports.__wbindgen_string_get=exports.__wbindgen_is_symbol=exports.__wbindgen_boolean_get=exports.__wbindgen_is_undefined=exports.__wbindgen_is_null=exports.__wbindgen_number_get=exports.__wbindgen_object_drop_ref=exports.__widl_f_data_ImageData=exports.__widl_f_height_ImageData=exports.__widl_f_width_ImageData=exports.__widl_f_new_with_u8_clamped_array_and_sh_ImageData=exports.segment_image=exports.default=void 0;var _=e(require("./pkg/web_slic_bg.wasm"));function e(_){return _&&_.__esModule?_:{default:_}}var t=_.default;exports.default=t;var a=_.default.segment_image;exports.segment_image=a;var n=_.default.__widl_f_new_with_u8_clamped_array_and_sh_ImageData;exports.__widl_f_new_with_u8_clamped_array_and_sh_ImageData=n;var d=_.default.__widl_f_width_ImageData;exports.__widl_f_width_ImageData=d;var r=_.default.__widl_f_height_ImageData;exports.__widl_f_height_ImageData=r;var i=_.default.__widl_f_data_ImageData;exports.__widl_f_data_ImageData=i;var g=_.default.__wbindgen_object_drop_ref;exports.__wbindgen_object_drop_ref=g;var s=_.default.__wbindgen_number_get;exports.__wbindgen_number_get=s;var o=_.default.__wbindgen_is_null;exports.__wbindgen_is_null=o;var w=_.default.__wbindgen_is_undefined;exports.__wbindgen_is_undefined=w;var l=_.default.__wbindgen_boolean_get;exports.__wbindgen_boolean_get=l;var b=_.default.__wbindgen_is_symbol;exports.__wbindgen_is_symbol=b;var f=_.default.__wbindgen_string_get;exports.__wbindgen_string_get=f;var p=_.default.__wbindgen_throw;exports.__wbindgen_throw=p;
},{"./pkg/web_slic_bg.wasm":"HsGZ"}],"Focm":[function(require,module,exports) {
"use strict";var e=t(require("../crate/Cargo.toml"));function t(e){return e&&e.__esModule?e:{default:e}}var a=document.querySelector("img"),n=document.querySelector("canvas"),u=n.getContext("2d"),c=document.querySelector("#segments-input"),o=document.querySelector("#compactness-input"),r=document.querySelector("#segments-label"),m=document.querySelector("#compactness-label");function l(){u.canvas.width=a.width,u.canvas.height=a.height+4,u.drawImage(a,0,0,a.width,a.height);var t=u.getImageData(0,0,a.width,a.height);u.putImageData(t,0,0),r.innerText="Segments (8-512): "+c.value,m.innerText="Compactness (1-50): "+o.value,t=e.default.segment_image(+c.value,+o.value,t),u.putImageData(t,0,0)}c.onchange=l,o.onchange=l,a.complete?l():a.onload=l;
},{"../crate/Cargo.toml":"SgWw"}],"3Fhe":[function(require,module,exports) {
var t=null;function r(){return t||(t=e()),t}function e(){try{throw new Error}catch(r){var t=(""+r.stack).match(/(https?|file|ftp):\/\/[^)\n]+/g);if(t)return n(t[0])}return"/"}function n(t){return(""+t).replace(/^((?:https?|file|ftp):\/\/.+)\/[^\/]+$/,"$1")+"/"}exports.getBundleURL=r,exports.getBaseURL=n;
},{}],"21/1":[function(require,module,exports) {
var r=require("./bundle-url").getBundleURL;function e(r){Array.isArray(r)||(r=[r]);var e=r[r.length-1];try{return Promise.resolve(require(e))}catch(n){if("MODULE_NOT_FOUND"===n.code)return new u(function(n,i){t(r.slice(0,-1)).then(function(){return require(e)}).then(n,i)});throw n}}function t(r){return Promise.all(r.map(s))}var n={};function i(r,e){n[r]=e}module.exports=exports=e,exports.load=t,exports.register=i;var o={};function s(e){var t;if(Array.isArray(e)&&(t=e[1],e=e[0]),o[e])return o[e];var i=(e.substring(e.lastIndexOf(".")+1,e.length)||e).toLowerCase(),s=n[i];return s?o[e]=s(r()+e).then(function(r){return r&&module.bundle.register(t,r),r}):void 0}function u(r){this.executor=r,this.promise=null}u.prototype.then=function(r,e){return null===this.promise&&(this.promise=new Promise(this.executor)),this.promise.then(r,e)},u.prototype.catch=function(r){return null===this.promise&&(this.promise=new Promise(this.executor)),this.promise.catch(r)};
},{"./bundle-url":"3Fhe"}],"fISM":[function(require,module,exports) {
var t;const n={},e=[{obj:void 0},{obj:null},{obj:!0},{obj:!1}];let r=e.length;function o(t){r===e.length&&e.push(e.length+1);const n=r,o=e[n];return r=o,e[n]={obj:t,cnt:1},n<<1}const i=[];function u(t){if(1==(1&t))return i[t>>1];return e[t>>1].obj}function a(t){if((t>>=1)<4)return;let n=e[t];n.cnt-=1,n.cnt>0||(e[t]=r,r=t)}function c(t){const n=u(t);return a(t),n}n.segment_image=function(n,e,r){return c(t.segment_image(o(n),o(e),o(r)))};let f=null;function _(){return null!==f&&f.buffer===t.memory.buffer||(f=new Uint32Array(t.memory.buffer)),f}let l=null;function s(){return null!==l&&l.buffer===t.memory.buffer||(l=new Uint8ClampedArray(t.memory.buffer)),l}function g(t,n){return s().subarray(t/1,t/1+n)}function b(t,n){for(;t;){let e=Object.getOwnPropertyDescriptor(t,n);if(e)return e;t=Object.getPrototypeOf(t)}throw new Error(`descriptor for id='${n}' not found`)}n.__widl_f_new_with_u8_clamped_array_and_sh_ImageData=function(t,n,e,r,i){let u=g(t,n);try{return o(new ImageData(u,e,r))}catch(a){const t=_();t[i/4]=1,t[i/4+1]=o(a)}};const d=b(ImageData.prototype,"width").get||function(){throw new Error("wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(ImageData.prototype, 'width').get does not exist")};n.__widl_f_width_ImageData=function(t){return d.call(u(t))};const w=b(ImageData.prototype,"height").get||function(){throw new Error("wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(ImageData.prototype, 'height').get does not exist")};n.__widl_f_height_ImageData=function(t){return w.call(u(t))};const m=b(ImageData.prototype,"data").get||function(){throw new Error("wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(ImageData.prototype, 'data').get does not exist")};let y=null;function h(){return null!==y&&y.buffer===t.memory.buffer||(y=new Uint8Array(t.memory.buffer)),y}function p(n){const e=t.__wbindgen_malloc(1*n.length);return h().set(n,e/1),[e,n.length]}n.__widl_f_data_ImageData=function(t,n){const[e,r]=p(m.call(u(n))),o=_();o[t/4]=e,o[t/4+1]=r},n.__wbindgen_object_drop_ref=function(t){a(t)},n.__wbindgen_number_get=function(t,n){let e=u(t);return"number"==typeof e?e:(h()[n]=1,0)},n.__wbindgen_is_null=function(t){return null===u(t)?1:0},n.__wbindgen_is_undefined=function(t){return void 0===u(t)?1:0},n.__wbindgen_boolean_get=function(t){let n=u(t);return"boolean"==typeof n?n?1:0:2},n.__wbindgen_is_symbol=function(t){return"symbol"==typeof u(t)?1:0};let D=new TextEncoder("utf-8");function I(n){const e=D.encode(n),r=t.__wbindgen_malloc(e.length);return h().set(e,r),[r,e.length]}n.__wbindgen_string_get=function(t,n){let e=u(t);if("string"!=typeof e)return 0;const[r,o]=I(e);return _()[n/4]=o,r};let O=new TextDecoder("utf-8");function j(t,n){return O.decode(h().subarray(t,t+n))}function x(e){const r=fetch(e);let o;return(o="function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(r,{"./web_slic":n}):r.then(t=>t.arrayBuffer()).then(t=>WebAssembly.instantiate(t,{"./web_slic":n}))).then(({instance:n})=>{t=x.wasm=n.exports})}n.__wbindgen_throw=function(t,n){throw new Error(j(t,n))};const A=Object.assign(x,n);module.exports=function(t){return A(t).then(()=>n)};
},{}],0:[function(require,module,exports) {
var b=require("21/1");b.register("wasm",require("fISM"));b.load([["web_slic_bg.ffb863b6.wasm","HsGZ"]]).then(function(){require("Focm");});
},{}]},{},[0], null)
//# sourceMappingURL=/web.6dea8736.map
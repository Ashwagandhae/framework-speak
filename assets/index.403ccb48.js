import{S as B,i as F,s as $,e as m,a as y,t as U,b as d,c as D,d as c,f as b,l as S,g as C,n as R,h as P,r as z,j as K}from"./vendor.e1d2345f.js";const V=function(){const t=document.createElement("link").relList;if(t&&t.supports&&t.supports("modulepreload"))return;for(const n of document.querySelectorAll('link[rel="modulepreload"]'))a(n);new MutationObserver(n=>{for(const s of n)if(s.type==="childList")for(const o of s.addedNodes)o.tagName==="LINK"&&o.rel==="modulepreload"&&a(o)}).observe(document,{childList:!0,subtree:!0});function r(n){const s={};return n.integrity&&(s.integrity=n.integrity),n.referrerpolicy&&(s.referrerPolicy=n.referrerpolicy),n.crossorigin==="use-credentials"?s.credentials="include":n.crossorigin==="anonymous"?s.credentials="omit":s.credentials="same-origin",s}function a(n){if(n.ep)return;n.ep=!0;const s=r(n);fetch(n.href,s)}};V();let l;const j=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});j.decode();let h=new Uint8Array;function g(){return h.byteLength===0&&(h=new Uint8Array(l.memory.buffer)),h}function x(e,t){return j.decode(g().subarray(e,e+t))}let W=0;const A=new TextEncoder("utf-8"),G=typeof A.encodeInto=="function"?function(e,t){return A.encodeInto(e,t)}:function(e,t){const r=A.encode(e);return t.set(r),{read:e.length,written:r.length}};function H(e,t,r){if(r===void 0){const i=A.encode(e),u=t(i.length);return g().subarray(u,u+i.length).set(i),W=i.length,u}let a=e.length,n=t(a);const s=g();let o=0;for(;o<a;o++){const i=e.charCodeAt(o);if(i>127)break;s[n+o]=i}if(o!==a){o!==0&&(e=e.slice(o)),n=r(n,a,a=o+e.length*3);const i=g().subarray(n+o,n+a);o+=G(e,i).written}return W=o,n}let v=new Int32Array;function q(){return v.byteLength===0&&(v=new Int32Array(l.memory.buffer)),v}class I{static __wrap(t){const r=Object.create(I.prototype);return r.ptr=t,r}__destroy_into_raw(){const t=this.ptr;return this.ptr=0,t}free(){const t=this.__destroy_into_raw();l.__wbg_frameworkreplacer_free(t)}constructor(){const t=l.frameworkreplacer_new();return I.__wrap(t)}replace(t,r){try{const s=l.__wbindgen_add_to_stack_pointer(-16),o=H(t,l.__wbindgen_malloc,l.__wbindgen_realloc),i=W;l.frameworkreplacer_replace(s,this.ptr,o,i,r);var a=q()[s/4+0],n=q()[s/4+1];return x(a,n)}finally{l.__wbindgen_add_to_stack_pointer(16),l.__wbindgen_free(a,n)}}}async function J(e,t){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,t)}catch(a){if(e.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",a);else throw a}const r=await e.arrayBuffer();return await WebAssembly.instantiate(r,t)}else{const r=await WebAssembly.instantiate(e,t);return r instanceof WebAssembly.Instance?{instance:r,module:e}:r}}function Q(){const e={};return e.wbg={},e.wbg.__wbindgen_throw=function(t,r){throw new Error(x(t,r))},e}function X(e,t){return l=e.exports,N.__wbindgen_wasm_module=t,v=new Int32Array,h=new Uint8Array,l}async function N(e){typeof e=="undefined"&&(e=new URL("/framework-speak/assets/vite_wasm_functions_bg.6b833ed7.wasm",self.location));const t=Q();(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:r,module:a}=await J(await e,t);return X(r,a)}function Y(e){let t,r,a,n,s,o,i,u,f,M,p,E,k,L,T,O;return{c(){t=m("main"),r=m("h1"),r.textContent="framework-speak.js",a=y(),n=m("label"),n.textContent="Temperature",s=y(),o=m("p"),i=U(e[1]),u=y(),f=m("input"),M=y(),p=m("textarea"),E=y(),k=m("p"),L=U(e[2]),d(n,"for","temp"),d(f,"type","range"),d(f,"min","0"),d(f,"max","1"),d(f,"step","0.01"),d(p,"placeholder","Type here"),d(p,"rows","10"),d(p,"cols","50"),d(p,"class","svelte-1wo6u7r")},m(_,w){D(_,t,w),c(t,r),c(t,a),c(t,n),c(t,s),c(t,o),c(o,i),c(t,u),c(t,f),b(f,e[1]),c(t,M),c(t,p),b(p,e[0]),c(t,E),c(t,k),c(k,L),T||(O=[S(f,"change",e[3]),S(f,"input",e[3]),S(p,"input",e[4])],T=!0)},p(_,[w]){w&2&&C(i,_[1]),w&2&&b(f,_[1]),w&1&&b(p,_[0]),w&4&&C(L,_[2])},i:R,o:R,d(_){_&&P(t),T=!1,z(O)}}}function Z(e,t,r){let a;const n=new I;let s="",o=.2;function i(){o=K(this.value),r(1,o)}function u(){s=this.value,r(0,s)}return e.$$.update=()=>{e.$$.dirty&3&&r(2,a=n.replace(s,o))},[s,o,a,i,u]}class ee extends B{constructor(t){super();F(this,t,Z,Y,$,{})}}const te=async()=>{const e=performance.now();await N();const t=performance.now();console.log(`Call to wasm init took ${t-e} milliseconds`),new ee({target:document.getElementById("app")})};te();
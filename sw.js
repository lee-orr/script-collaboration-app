if(!self.define){let e,s={};const n=(n,i)=>(n=new URL(n+".js",i).href,s[n]||new Promise((s=>{if("document"in self){const e=document.createElement("script");e.src=n,e.onload=s,document.head.appendChild(e)}else e=n,importScripts(n),s()})).then((()=>{let e=s[n];if(!e)throw new Error(`Module ${n} didn’t register its module`);return e})));self.define=(i,r)=>{const o=e||("document"in self?document.currentScript.src:"")||location.href;if(s[o])return;let f={};const l=e=>n(e,o),t={module:{uri:o},exports:f,require:l};s[o]=Promise.all(i.map((e=>t[e]||l(e)))).then((e=>(r(...e),f)))}}define(["./workbox-3ea082d2"],(function(e){"use strict";self.skipWaiting(),e.clientsClaim(),e.precacheAndRoute([{url:"assets/Button.0249f3d8.js",revision:null},{url:"assets/Host.1c1c8d51.js",revision:null},{url:"assets/index.c810bfb7.js",revision:null},{url:"assets/index.f6fee4be.css",revision:null},{url:"assets/Input.db985ffe.js",revision:null},{url:"assets/Join.f2be4984.js",revision:null},{url:"assets/Menu.f3799bf7.js",revision:null},{url:"assets/Session.ebd964f3.js",revision:null},{url:"index.html",revision:"f19fa6aa2a6c0b078d11c1c116866d4f"},{url:"favicon.png",revision:"ddc7574953929ad7d634cff3303ba0e3"},{url:"robots.txt",revision:"f77c87f977e0fcce05a6df46c885a129"},{url:"apple-touch-icon.png",revision:"78dafd7f571c791fce6e6d465c936f21"},{url:"android-chrome-192x192.png",revision:"7544febbd35ebecee9ad709ac9d166d8"},{url:"android-chrome-512x512.png",revision:"6d627213d96bfde91c649fd1274e6baf"},{url:"icons/arrow-left.svg",revision:"5344bb894484f478b2ad912ef0440436"},{url:"fonts/Inter-Bold.woff2",revision:"444a7284663a3bc886683eb81450b294"},{url:"fonts/Inter-Medium.woff2",revision:"75db5319e7e87c587019a5df08d7272c"},{url:"fonts/Inter-Regular.woff2",revision:"dc131113894217b5031000575d9de002"},{url:"manifest.webmanifest",revision:"501ceccfb5623062ef5ee922ac668f76"}],{}),e.cleanupOutdatedCaches(),e.registerRoute(new e.NavigationRoute(e.createHandlerBoundToURL("index.html")))}));

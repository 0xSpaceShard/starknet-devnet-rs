"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[5282],{6839:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>s,contentTitle:()=>a,default:()=>u,frontMatter:()=>i,metadata:()=>c,toc:()=>d});var o=t(4848),r=t(8453);const i={},a="Forking",c={id:"forking",title:"Forking",description:"To interact with contracts deployed on mainnet or testnet, you can use forking. Simulate the origin and experiment with it locally, making no changes to the origin itself.",source:"@site/versioned_docs/version-0.2.0/forking.md",sourceDirName:".",slug:"/forking",permalink:"/starknet-devnet-rs/docs/forking",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet-rs/blob/master/website/versioned_docs/version-0.2.0/forking.md",tags:[],version:"0.2.0",frontMatter:{},sidebar:"docSidebar",previous:{title:"Examples",permalink:"/starknet-devnet-rs/docs/examples"},next:{title:"Gas price modification",permalink:"/starknet-devnet-rs/docs/gas"}},s={},d=[{value:"Account impersonation",id:"account-impersonation",level:2},{value:"Deploying an undeclared account",id:"deploying-an-undeclared-account",level:2}];function l(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",p:"p",pre:"pre",...(0,r.R)(),...e.components};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(n.h1,{id:"forking",children:"Forking"}),"\n",(0,o.jsx)(n.p,{children:"To interact with contracts deployed on mainnet or testnet, you can use forking. Simulate the origin and experiment with it locally, making no changes to the origin itself."}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:"$ starknet-devnet --fork-network <URL> [--fork-block <BLOCK_NUMBER>]\n"})}),"\n",(0,o.jsxs)(n.p,{children:["The value passed to ",(0,o.jsx)(n.code,{children:"--fork-network"})," should be the URL to a Starknet JSON-RPC API provider. Specifying a ",(0,o.jsx)(n.code,{children:"--fork-block"})," is optional; it defaults to the ",(0,o.jsx)(n.code,{children:'"latest"'})," block at the time of Devnet's start-up. All calls will first try Devnet's state and then fall back to the forking block."]}),"\n",(0,o.jsx)(n.h2,{id:"account-impersonation",children:"Account impersonation"}),"\n",(0,o.jsxs)(n.p,{children:[(0,o.jsx)(n.a,{href:"./account-impersonation",children:"Here"})," you can read more about acting as an account deployed on the origin."]}),"\n",(0,o.jsx)(n.h2,{id:"deploying-an-undeclared-account",children:"Deploying an undeclared account"}),"\n",(0,o.jsxs)(n.p,{children:[(0,o.jsx)(n.a,{href:"./predeployed#deploying-an-undeclared-account",children:"Here"})," you can read about deploying an account not declared on Devnet."]})]})}function u(e={}){const{wrapper:n}={...(0,r.R)(),...e.components};return n?(0,o.jsx)(n,{...e,children:(0,o.jsx)(l,{...e})}):l(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>a,x:()=>c});var o=t(6540);const r={},i=o.createContext(r);function a(e){const n=o.useContext(i);return o.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function c(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:a(e.components),o.createElement(i.Provider,{value:n},e.children)}}}]);
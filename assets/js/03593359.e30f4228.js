"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[6114],{4025:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>o,default:()=>h,frontMatter:()=>s,metadata:()=>d,toc:()=>a});var r=n(4848),i=n(8453);const s={},o="Restrictive mode",d={id:"restrictive",title:"Restrictive mode",description:"The --restrictive-mode argument enables a restrictive mode for Devnet, allowing you to specify methods that are forbidden during execution. This option ensures that certain operations are restricted, enhancing control over Devnet's behavior. When a user sends a request to one of the restricted methods, Devnet will return either a JSON-RPC error with code -32604 or, if the method was targeted directly via the HTTP endpoint, a response with status 403.",source:"@site/versioned_docs/version-0.2.1/restrictive.md",sourceDirName:".",slug:"/restrictive",permalink:"/starknet-devnet/docs/0.2.1/restrictive",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet/blob/master/website/versioned_docs/version-0.2.1/restrictive.md",tags:[],version:"0.2.1",frontMatter:{},sidebar:"docSidebar",previous:{title:"Predeployed contracts",permalink:"/starknet-devnet/docs/0.2.1/predeployed"},next:{title:"Server config",permalink:"/starknet-devnet/docs/0.2.1/server-config"}},c={},a=[{value:"Default restricted methods",id:"default-restricted-methods",level:2},{value:"Usage",id:"usage",level:2},{value:"With default methods",id:"with-default-methods",level:3},{value:"With a list of methods",id:"with-a-list-of-methods",level:3}];function l(e){const t={code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,i.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(t.h1,{id:"restrictive-mode",children:"Restrictive mode"}),"\n",(0,r.jsx)(t.p,{children:"The --restrictive-mode argument enables a restrictive mode for Devnet, allowing you to specify methods that are forbidden during execution. This option ensures that certain operations are restricted, enhancing control over Devnet's behavior. When a user sends a request to one of the restricted methods, Devnet will return either a JSON-RPC error with code -32604 or, if the method was targeted directly via the HTTP endpoint, a response with status 403."}),"\n",(0,r.jsx)(t.h2,{id:"default-restricted-methods",children:"Default restricted methods"}),"\n",(0,r.jsx)(t.p,{children:"When no methods are specified, the following default methods will be restricted and their HTTP endpoints counterparts (if any):"}),"\n",(0,r.jsxs)(t.ul,{children:["\n",(0,r.jsx)(t.li,{children:"devnet_mint"}),"\n",(0,r.jsx)(t.li,{children:"devnet_load"}),"\n",(0,r.jsx)(t.li,{children:"devnet_restart"}),"\n",(0,r.jsx)(t.li,{children:"devnet_createBlock"}),"\n",(0,r.jsx)(t.li,{children:"devnet_abortBlocks"}),"\n",(0,r.jsx)(t.li,{children:"devnet_impersonateAccount"}),"\n",(0,r.jsx)(t.li,{children:"devnet_autoImpersonate"}),"\n",(0,r.jsx)(t.li,{children:"devnet_getPredeployedAccounts"}),"\n"]}),"\n",(0,r.jsx)(t.h2,{id:"usage",children:"Usage"}),"\n",(0,r.jsx)(t.h3,{id:"with-default-methods",children:"With default methods"}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{children:"$ starknet-devnet --restrictive-mode\n"})}),"\n",(0,r.jsx)(t.h3,{id:"with-a-list-of-methods",children:"With a list of methods"}),"\n",(0,r.jsx)(t.p,{children:"Note! Devnet will fail to start if any of the methods/routes is misspelled."}),"\n",(0,r.jsx)(t.pre,{children:(0,r.jsx)(t.code,{children:"$ starknet-devnet --restrictive-mode devnet_dump devnet_config\n"})})]})}function h(e={}){const{wrapper:t}={...(0,i.R)(),...e.components};return t?(0,r.jsx)(t,{...e,children:(0,r.jsx)(l,{...e})}):l(e)}},8453:(e,t,n)=>{n.d(t,{R:()=>o,x:()=>d});var r=n(6540);const i={},s=r.createContext(i);function o(e){const t=r.useContext(s);return r.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function d(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:o(e.components),r.createElement(s.Provider,{value:t},e.children)}}}]);
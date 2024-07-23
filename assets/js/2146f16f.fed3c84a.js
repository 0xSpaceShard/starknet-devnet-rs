"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[6773],{1017:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>c,contentTitle:()=>i,default:()=>p,frontMatter:()=>a,metadata:()=>r,toc:()=>d});var o=t(4848),s=t(8453);const a={},i="Account impersonation",r={id:"account-impersonation",title:"Account impersonation",description:"This page is about account impersonation. To read about account class selection and deployment, click here.",source:"@site/versioned_docs/version-0.0.7/account-impersonation.md",sourceDirName:".",slug:"/account-impersonation",permalink:"/starknet-devnet-rs/docs/0.0.7/account-impersonation",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet-rs/blob/master/website/versioned_docs/version-0.0.7/account-impersonation.md",tags:[],version:"0.0.7",frontMatter:{},sidebar:"docSidebar",previous:{title:"CLI options",permalink:"/starknet-devnet-rs/docs/0.0.7/running/cli"},next:{title:"API",permalink:"/starknet-devnet-rs/docs/0.0.7/api"}},c={},d=[{value:"API",id:"api",level:2},{value:"devnet_impersonateAccount",id:"devnet_impersonateaccount",level:3},{value:"devnet_stopImpersonateAccount",id:"devnet_stopimpersonateaccount",level:3},{value:"devnet_autoImpersonate",id:"devnet_autoimpersonate",level:3},{value:"devnet_stopAutoImpersonate",id:"devnet_stopautoimpersonate",level:3}];function l(e){const n={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,s.R)(),...e.components};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(n.h1,{id:"account-impersonation",children:"Account impersonation"}),"\n",(0,o.jsx)(n.admonition,{type:"info",children:(0,o.jsxs)(n.p,{children:["This page is about account impersonation. To read about account class selection and deployment, click ",(0,o.jsx)(n.a,{href:"./predeployed",children:"here"}),"."]})}),"\n",(0,o.jsxs)(n.p,{children:["Devnet allows you to use impersonated account from mainnet/testnet. This means that a transaction sent from an impersonated account will not fail with an invalid signature error. In the general case, a transaction sent with an account that is not in the local state fails with the aforementioned error. For impersonation to work, Devnet needs to be run in ",(0,o.jsx)(n.a,{href:"/starknet-devnet-rs/docs/0.0.7/forking",children:"forking mode"}),"."]}),"\n",(0,o.jsx)(n.admonition,{title:"Caveat",type:"warning",children:(0,o.jsxs)(n.ul,{children:["\n",(0,o.jsxs)(n.li,{children:["Only ",(0,o.jsx)(n.code,{children:"INVOKE"})," and ",(0,o.jsx)(n.code,{children:"DECLARE"})," transactions are supported. ",(0,o.jsx)(n.code,{children:"DEPLOY_ACCOUNT"})," transaction is not supported, but you can create an ",(0,o.jsx)(n.code,{children:"INVOKE"})," transaction to UDC."]}),"\n",(0,o.jsx)(n.li,{children:"Overall fee, for transactions sent with an impersonated account, will be lower compared to normal transactions. The reason is that validation part is skipped."}),"\n",(0,o.jsxs)(n.li,{children:["The most common way of sending a transaction is via starknet-rs/starknet.js or starkli. Trying to send with an account that ",(0,o.jsx)(n.strong,{children:"does not"})," exist even in the origin network will return an error:","\n",(0,o.jsxs)(n.ul,{children:["\n",(0,o.jsxs)(n.li,{children:["In transaction construction, if account nonce is not hardcoded, Devnet is queried and returns ",(0,o.jsx)(n.code,{children:"ContractNotFound"}),"."]}),"\n",(0,o.jsxs)(n.li,{children:["Otherwise the nonce fetching part is skipped and ",(0,o.jsx)(n.code,{children:"InsufficientAccountBalance"})," is returned."]}),"\n"]}),"\n"]}),"\n"]})}),"\n",(0,o.jsxs)(n.p,{children:["Users can disable account impersonation by starting Devnet with CLI flag ",(0,o.jsx)(n.code,{children:"--disable-account-impersonation"})," or by setting environment variable ",(0,o.jsx)(n.code,{children:"DISABLE_ACCOUNT_IMPERSONATION"}),". Every subsequent JSON-RPC impersonation request will return an error. This feature can be used in CTFs to prevent participants from easily solving the task."]}),"\n",(0,o.jsx)(n.h2,{id:"api",children:"API"}),"\n",(0,o.jsx)(n.p,{children:"Account impersonation follows JSON-RPC method specification. Each method returns an empty response:"}),"\n",(0,o.jsx)(n.h3,{id:"devnet_impersonateaccount",children:"devnet_impersonateAccount"}),"\n",(0,o.jsx)(n.p,{children:"Impersonates a specific account address nonexistent in the local state."}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{className:"language-js",children:'{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_impersonateAccount",\n    "params": {\n        "account_address": "0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7"\n    }\n}\n'})}),"\n",(0,o.jsx)(n.h3,{id:"devnet_stopimpersonateaccount",children:"devnet_stopImpersonateAccount"}),"\n",(0,o.jsx)(n.p,{children:"Stops the impersonation of an account previously marked for impersonation."}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{className:"language-js",children:'{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_stopImpersonateAccount",\n    "params": {\n        "account_address": "0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7"\n    }\n}\n'})}),"\n",(0,o.jsx)(n.h3,{id:"devnet_autoimpersonate",children:"devnet_autoImpersonate"}),"\n",(0,o.jsx)(n.p,{children:"Enables automatic account impersonation. Every account that does not exist in the local state will be impersonated."}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{className:"language-js",children:'{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_autoImpersonate",\n    "params": {}\n}\n'})}),"\n",(0,o.jsx)(n.h3,{id:"devnet_stopautoimpersonate",children:"devnet_stopAutoImpersonate"}),"\n",(0,o.jsxs)(n.p,{children:["Stops the effect of ",(0,o.jsx)(n.a,{href:"#devnet_autoimpersonate",children:"automatic impersonation"}),"."]}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{className:"language-js",children:'{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_stopAutoImpersonate",\n    "params": {}\n}\n'})})]})}function p(e={}){const{wrapper:n}={...(0,s.R)(),...e.components};return n?(0,o.jsx)(n,{...e,children:(0,o.jsx)(l,{...e})}):l(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>i,x:()=>r});var o=t(6540);const s={},a=o.createContext(s);function i(e){const n=o.useContext(a);return o.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function r(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(s):e.components||s:i(e.components),o.createElement(a.Provider,{value:n},e.children)}}}]);
"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[9687],{1286:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>d,contentTitle:()=>r,default:()=>l,frontMatter:()=>s,metadata:()=>c,toc:()=>o});var i=n(4848),a=n(8453);const s={sidebar_position:3},r="API",c={id:"api",title:"API",description:"JSON-RPC API",source:"@site/versioned_docs/version-0.2.2/api.md",sourceDirName:".",slug:"/api",permalink:"/starknet-devnet/docs/0.2.2/api",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet/blob/master/website/versioned_docs/version-0.2.2/api.md",tags:[],version:"0.2.2",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"docSidebar",previous:{title:"CLI options",permalink:"/starknet-devnet/docs/0.2.2/running/cli"},next:{title:"Account impersonation",permalink:"/starknet-devnet/docs/0.2.2/account-impersonation"}},d={},o=[{value:"JSON-RPC API",id:"json-rpc-api",level:2},{value:"Starknet API",id:"starknet-api",level:3},{value:"Devnet API",id:"devnet-api",level:3},{value:"Healthcheck",id:"healthcheck",level:4},{value:"Interacting with Devnet in JavaScript and TypeScript",id:"interacting-with-devnet-in-javascript-and-typescript",level:2},{value:"Config API",id:"config-api",level:2}];function h(e){const t={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",h3:"h3",h4:"h4",p:"p",pre:"pre",...(0,a.R)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(t.h1,{id:"api",children:"API"}),"\n",(0,i.jsx)(t.h2,{id:"json-rpc-api",children:"JSON-RPC API"}),"\n",(0,i.jsxs)(t.p,{children:["Both Starknet's and Devnet's JSON-RPC API are reachable at ",(0,i.jsx)(t.code,{children:"/rpc"})," and ",(0,i.jsx)(t.code,{children:"/"}),". E.g. if spawning Devnet with default settings, these URLs are functionally equivalent: ",(0,i.jsx)(t.code,{children:"http://127.0.0.1:5050/rpc"})," and ",(0,i.jsx)(t.code,{children:"http://127.0.0.1:5050/"}),". The difference between these two groups of methods is their prefix: ",(0,i.jsx)(t.code,{children:"starknet_"})," (e.g. ",(0,i.jsx)(t.code,{children:"starknet_getNonce"}),") and ",(0,i.jsx)(t.code,{children:"devnet_"})," (e.g. ",(0,i.jsx)(t.code,{children:"devnet_mint"}),")."]}),"\n",(0,i.jsx)(t.h3,{id:"starknet-api",children:"Starknet API"}),"\n",(0,i.jsxs)(t.p,{children:["Unlike Pythonic Devnet, which also supported Starknet's gateway and feeder gateway API, Devnet in Rust supports ",(0,i.jsx)(t.a,{href:"https://github.com/starkware-libs/starknet-specs/tree/master/api",children:"Starknet's JSON-RPC API"}),". Since JSON-RPC v0.6.0, to find out which JSON-RPC version is supported by which Devnet version, check out the ",(0,i.jsx)(t.a,{href:"https://github.com/0xSpaceShard/starknet-devnet/releases",children:"releases page"}),"."]}),"\n",(0,i.jsx)(t.h3,{id:"devnet-api",children:"Devnet API"}),"\n",(0,i.jsxs)(t.p,{children:["Devnet has many additional features which are available via their own endpoints and JSON-RPC. The RPC methods are documented throughout the documentation in their corresponding pages, but are also aggregated ",(0,i.jsx)(t.a,{href:"https://github.com/0xSpaceShard/starknet-devnet/blob/main/website/static/devnet_api.json",children:"here"}),"."]}),"\n",(0,i.jsx)(t.admonition,{title:"Deprecation notice",type:"warning",children:(0,i.jsxs)(t.p,{children:["New features are only supported as part of the JSON-RPC API. Older non-RPC requests are still supported, but considered deprecated - they will be removed in the future, except the ",(0,i.jsx)(t.a,{href:"#healthcheck",children:"healthcheck endpoint"}),"."]})}),"\n",(0,i.jsx)(t.h4,{id:"healthcheck",children:"Healthcheck"}),"\n",(0,i.jsxs)(t.p,{children:["To check if a Devnet instance is alive, send an HTTP request ",(0,i.jsx)(t.code,{children:"GET /is_alive"}),". If alive, the Devnet will reply with a ",(0,i.jsx)(t.code,{children:"200 OK"})," and an appropriate message."]}),"\n",(0,i.jsx)(t.h2,{id:"interacting-with-devnet-in-javascript-and-typescript",children:"Interacting with Devnet in JavaScript and TypeScript"}),"\n",(0,i.jsxs)(t.p,{children:["To spawn Devnet and interact with it using the ",(0,i.jsx)(t.a,{href:"#devnet-api",children:"Devnet API"}),", you can use ",(0,i.jsx)(t.a,{href:"https://github.com/0xSpaceShard/starknet-devnet-js/",children:(0,i.jsx)(t.code,{children:"starknet-devnet-js"})}),". This can be especially useful in achieving ",(0,i.jsx)(t.a,{href:"/starknet-devnet/docs/0.2.2/postman#l1-l2-interaction-via-postman",children:"L1-L2 communication"}),"."]}),"\n",(0,i.jsxs)(t.p,{children:["To interact with Devnet using the ",(0,i.jsx)(t.a,{href:"#starknet-api",children:"Starknet API"}),", use ",(0,i.jsx)(t.a,{href:"https://www.starknetjs.com/",children:"starknet.js"}),"."]}),"\n",(0,i.jsx)(t.h2,{id:"config-api",children:"Config API"}),"\n",(0,i.jsxs)(t.p,{children:["To retrieve the current configuration of Devnet, as specified via ",(0,i.jsx)(t.a,{href:"/starknet-devnet/docs/0.2.2/running/cli",children:"CLI"})," and later requests, send a ",(0,i.jsx)(t.code,{children:"GET"})," request to ",(0,i.jsx)(t.code,{children:"/config"})," or ",(0,i.jsx)(t.code,{children:"JSON-RPC"})," request with method name ",(0,i.jsx)(t.code,{children:"devnet_getConfig"}),". Example response is attached below. It can be interpreted as a JSON mapping of CLI input parameters, both specified and default ones, with some irrelevant parameters omitted. So use ",(0,i.jsx)(t.code,{children:"starknet-devnet --help"})," to better understand the meaning of each value, though keep in mind that some of the parameters have slightly modified names."]}),"\n",(0,i.jsx)(t.pre,{children:(0,i.jsx)(t.code,{className:"language-json",children:'{\n  "seed": 4063802897,\n  "total_accounts": 10,\n  "account_contract_class_hash": "0x61dac032f228abef9c6626f995015233097ae253a7f72d68552db02f2971b8f",\n  "predeployed_accounts_initial_balance": "1000000000000000000000",\n  "start_time": null,\n  "gas_price_wei": 100000000000,\n  "gas_price_fri": 100000000000,\n  "data_gas_price_wei": 100000000000,\n  "data_gas_price_fri": 100000000000,\n  "chain_id": "SN_SEPOLIA",\n  "dump_on": "exit",\n  "dump_path": "dump_path.json",\n  "state_archive": "none",\n  "fork_config": {\n    "url": "http://rpc.pathfinder.equilibrium.co/integration-sepolia/rpc/v0_7",\n    "block_number": 26429\n  },\n  "server_config": {\n    "host": "127.0.0.1",\n    "port": 5050,\n    "timeout": 120,\n    "request_body_size_limit": 2000000,\n    "restricted_methods": null\n  },\n  "block_generation": null,\n  "lite_mode": false,\n  "eth_erc20_class_hash": "0x046ded64ae2dead6448e247234bab192a9c483644395b66f2155f2614e5804b0",\n  "strk_erc20_class_hash": "0x046ded64ae2dead6448e247234bab192a9c483644395b66f2155f2614e5804b0"\n}\n'})})]})}function l(e={}){const{wrapper:t}={...(0,a.R)(),...e.components};return t?(0,i.jsx)(t,{...e,children:(0,i.jsx)(h,{...e})}):h(e)}},8453:(e,t,n)=>{n.d(t,{R:()=>r,x:()=>c});var i=n(6540);const a={},s=i.createContext(a);function r(e){const t=i.useContext(s);return i.useMemo((function(){return"function"==typeof e?e(t):{...t,...e}}),[t,e])}function c(e){let t;return t=e.disableParentContext?"function"==typeof e.components?e.components(a):e.components||a:r(e.components),i.createElement(s.Provider,{value:t},e.children)}}}]);
"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[9378],{8032:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>c,contentTitle:()=>r,default:()=>p,frontMatter:()=>s,metadata:()=>o,toc:()=>d});var a=t(4848),i=t(8453);const s={},r="API",o={id:"api",title:"API",description:"Starknet API",source:"@site/versioned_docs/version-0.0.6/api.md",sourceDirName:".",slug:"/api",permalink:"/starknet-devnet/docs/0.0.6/api",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet/blob/master/website/versioned_docs/version-0.0.6/api.md",tags:[],version:"0.0.6",frontMatter:{},sidebar:"docSidebar",previous:{title:"Account impersonation",permalink:"/starknet-devnet/docs/0.0.6/account-impersonation"},next:{title:"Account balance",permalink:"/starknet-devnet/docs/0.0.6/balance"}},c={},d=[{value:"Starknet API",id:"starknet-api",level:2},{value:"Devnet API",id:"devnet-api",level:2},{value:"Config API",id:"config-api",level:2}];function l(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",p:"p",pre:"pre",...(0,i.R)(),...e.components};return(0,a.jsxs)(a.Fragment,{children:[(0,a.jsx)(n.h1,{id:"api",children:"API"}),"\n",(0,a.jsx)(n.h2,{id:"starknet-api",children:"Starknet API"}),"\n",(0,a.jsxs)(n.p,{children:["Unlike Pythonic Devnet, which also supported Starknet's gateway and feeder gateway API, Devnet in Rust supports Starknet's JSON-RPC API. Since JSON-RPC v0.6.0, to find out which JSON-RPC version is supported by which Devnet version, check out the ",(0,a.jsx)(n.a,{href:"https://github.com/0xSpaceShard/starknet-devnet/releases",children:"releases page"}),"."]}),"\n",(0,a.jsxs)(n.p,{children:["The JSON-RPC API is reachable via ",(0,a.jsx)(n.code,{children:"/rpc"})," and ",(0,a.jsx)(n.code,{children:"/"})," (e.g. if spawning Devnet with default settings, these URLs are functionally equivalent: ",(0,a.jsx)(n.code,{children:"http://127.0.0.1:5050/rpc"})," and ",(0,a.jsx)(n.code,{children:"http://127.0.0.1:5050/"}),")"]}),"\n",(0,a.jsx)(n.h2,{id:"devnet-api",children:"Devnet API"}),"\n",(0,a.jsx)(n.p,{children:"Devnet has many other functional features which are available via their own endpoints, which are all mentioned throughout the documentation."}),"\n",(0,a.jsx)(n.h2,{id:"config-api",children:"Config API"}),"\n",(0,a.jsxs)(n.p,{children:["To retrieve the current configuration of Devnet, send a ",(0,a.jsx)(n.code,{children:"GET"})," request to ",(0,a.jsx)(n.code,{children:"/config"}),". Example response is attached below. It can be interpreted as a JSON mapping of CLI input parameters, both specified and default ones, with some irrelevant parameters omitted. So use ",(0,a.jsx)(n.code,{children:"starknet-devnet --help"})," to better understand the meaning of each value, though keep in mind that some of the parameters have slightly modified names."]}),"\n",(0,a.jsx)(n.pre,{children:(0,a.jsx)(n.code,{className:"language-json",children:'{\n  "seed": 4063802897,\n  "total_accounts": 10,\n  "account_contract_class_hash": "0x61dac032f228abef9c6626f995015233097ae253a7f72d68552db02f2971b8f",\n  "predeployed_accounts_initial_balance": "1000000000000000000000",\n  "start_time": null,\n  "gas_price_wei": 100000000000,\n  "gas_price_strk": 100000000000,\n  "data_gas_price_wei": 100000000000,\n  "data_gas_price_strk": 100000000000,\n  "chain_id": "SN_SEPOLIA",\n  "dump_on": "exit",\n  "dump_path": "dump_path.json",\n  "state_archive": "none",\n  "fork_config": {\n    "url": "http://rpc.pathfinder.equilibrium.co/integration-sepolia/rpc/v0_7",\n    "block_number": 26429\n  },\n  "server_config": {\n    "host": "127.0.0.1",\n    "port": 5050,\n    "timeout": 120,\n    "request_body_size_limit": 2000000\n  },\n  "blocks_on_demand": false,\n  "lite_mode": false\n}\n'})})]})}function p(e={}){const{wrapper:n}={...(0,i.R)(),...e.components};return n?(0,a.jsx)(n,{...e,children:(0,a.jsx)(l,{...e})}):l(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>r,x:()=>o});var a=t(6540);const i={},s=a.createContext(i);function r(e){const n=a.useContext(s);return a.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function o(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:r(e.components),a.createElement(s.Provider,{value:n},e.children)}}}]);
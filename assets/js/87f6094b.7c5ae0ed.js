"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[9939],{1849:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>r,contentTitle:()=>d,default:()=>h,frontMatter:()=>a,metadata:()=>o,toc:()=>i});var c=t(4848),s=t(8453);const a={},d="Account balance",o={id:"balance",title:"Account balance",description:"Other than using prefunded predeployed accounts, you can also add funds to an account that you deployed yourself.",source:"@site/versioned_docs/version-0.2.3/balance.md",sourceDirName:".",slug:"/balance",permalink:"/starknet-devnet-rs/docs/0.2.3/balance",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet-rs/blob/master/website/versioned_docs/version-0.2.3/balance.md",tags:[],version:"0.2.3",frontMatter:{},sidebar:"docSidebar",previous:{title:"Account impersonation",permalink:"/starknet-devnet-rs/docs/0.2.3/account-impersonation"},next:{title:"Blocks",permalink:"/starknet-devnet-rs/docs/0.2.3/blocks"}},r={},i=[{value:"Mint token - Local faucet",id:"mint-token---local-faucet",level:2},{value:"Check balance",id:"check-balance",level:2}];function l(e){const n={code:"code",h1:"h1",h2:"h2",li:"li",p:"p",pre:"pre",ul:"ul",...(0,s.R)(),...e.components};return(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.h1,{id:"account-balance",children:"Account balance"}),"\n",(0,c.jsx)(n.p,{children:"Other than using prefunded predeployed accounts, you can also add funds to an account that you deployed yourself."}),"\n",(0,c.jsx)(n.p,{children:"Separate tokens use separate ERC20 contracts for minting and charging fees. These are the token contracts predeployed by Devnet and the addresses where they are located:"}),"\n",(0,c.jsxs)(n.ul,{children:["\n",(0,c.jsxs)(n.li,{children:["ETH: ",(0,c.jsx)(n.code,{children:"0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"})]}),"\n",(0,c.jsxs)(n.li,{children:["STRK: ",(0,c.jsx)(n.code,{children:"0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d"})]}),"\n"]}),"\n",(0,c.jsx)(n.h2,{id:"mint-token---local-faucet",children:"Mint token - Local faucet"}),"\n",(0,c.jsxs)(n.p,{children:["By sending a ",(0,c.jsx)(n.code,{children:"POST"})," request to ",(0,c.jsx)(n.code,{children:"/mint"})," or ",(0,c.jsx)(n.code,{children:"JSON-RPC"})," request with method name ",(0,c.jsx)(n.code,{children:"devnet_mint"})," for a token, you initiate a transaction on that token's ERC20 contract. The response contains the hash of this transaction, as well as the new balance after minting. The token is specified by providing the unit, and defaults to ",(0,c.jsx)(n.code,{children:"WEI"}),"."]}),"\n",(0,c.jsxs)(n.p,{children:["The value of ",(0,c.jsx)(n.code,{children:"amount"})," is in WEI or FRI. The precision is preserved if specifying an integer or a float whose fractional part is zero (e.g. ",(0,c.jsx)(n.code,{children:"1000.0"}),", ",(0,c.jsx)(n.code,{children:"1e21"}),"). If the fractional part is non-zero, the amount is truncated to the nearest integer (e.g. ",(0,c.jsx)(n.code,{children:"3.9"})," becomes ",(0,c.jsx)(n.code,{children:"3"})," and ",(0,c.jsx)(n.code,{children:"1.23e1"})," becomes ",(0,c.jsx)(n.code,{children:"12"}),")."]}),"\n",(0,c.jsx)(n.pre,{children:(0,c.jsx)(n.code,{children:'POST /mint\n{\n    "address": "0x6e3205f...",\n    "amount": 500000,\n    "unit": "WEI" | "FRI"\n}\n'})}),"\n",(0,c.jsx)(n.pre,{children:(0,c.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_mint",\n    "params": {\n        "address": "0x6e3205f...",\n        "amount": 500000,\n        "unit": "WEI" | "FRI"\n    }\n}\n'})}),"\n",(0,c.jsx)(n.p,{children:"Response:"}),"\n",(0,c.jsx)(n.pre,{children:(0,c.jsx)(n.code,{children:'{\n    "new_balance": 500000,\n    "unit": "WEI" | "FRI",\n    "tx_hash": "0xa24f23..."\n}\n'})}),"\n",(0,c.jsx)(n.h2,{id:"check-balance",children:"Check balance"}),"\n",(0,c.jsxs)(n.p,{children:["Check the balance of an address by sending a ",(0,c.jsx)(n.code,{children:"GET"})," request to ",(0,c.jsx)(n.code,{children:"/account_balance"}),". The address should be a 0x-prefixed hex string; ",(0,c.jsx)(n.code,{children:"unit"})," defaults to ",(0,c.jsx)(n.code,{children:"WEI"})," and ",(0,c.jsx)(n.code,{children:"block_tag"})," to ",(0,c.jsx)(n.code,{children:"latest"}),"."]}),"\n",(0,c.jsx)(n.pre,{children:(0,c.jsx)(n.code,{children:"GET /account_balance?address=<ADDRESS>[&unit=<FRI|WEI>][&block_tag=<latest|pending>]\n"})}),"\n",(0,c.jsx)(n.pre,{children:(0,c.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_getAccountBalance",\n    "params": {\n        "address": "0x6e3205f...",\n        "unit": "WEI" | "FRI",\n        "block_tag": "latest" | "pending"\n    }\n}\n'})})]})}function h(e={}){const{wrapper:n}={...(0,s.R)(),...e.components};return n?(0,c.jsx)(n,{...e,children:(0,c.jsx)(l,{...e})}):l(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>d,x:()=>o});var c=t(6540);const s={},a=c.createContext(s);function d(e){const n=c.useContext(a);return c.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function o(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(s):e.components||s:d(e.components),c.createElement(a.Provider,{value:n},e.children)}}}]);
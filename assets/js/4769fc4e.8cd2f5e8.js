"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[2627],{4570:(e,n,s)=>{s.r(n),s.d(n,{assets:()=>d,contentTitle:()=>o,default:()=>h,frontMatter:()=>a,metadata:()=>i,toc:()=>c});var t=s(4848),r=s(8453);const a={},o="L1-L2 interaction via Postman",i={id:"postman",title:"L1-L2 interaction via Postman",description:"Postman is a Starknet utility that allows testing L1-L2 interaction. Ensure you have an L1 node and a Devnet (L2 node) running, load a messaging contract, and flush the queue when needed. You can use starknet-hardhat-plugin to perform these actions, as witnessed in this example, or directly send requests to the endpoints specified below.",source:"@site/versioned_docs/version-0.0.6/postman.md",sourceDirName:".",slug:"/postman",permalink:"/starknet-devnet-rs/docs/0.0.6/postman",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet-rs/blob/master/website/versioned_docs/version-0.0.6/postman.md",tags:[],version:"0.0.6",frontMatter:{},sidebar:"docSidebar",previous:{title:"Lite mode",permalink:"/starknet-devnet-rs/docs/0.0.6/lite"},next:{title:"Predeployed contracts",permalink:"/starknet-devnet-rs/docs/0.0.6/predeployed"}},d={},c=[{value:"Load",id:"load",level:2},{value:"Flush",id:"flush",level:2},{value:"Disclaimer",id:"disclaimer",level:2},{value:"Mock transactions",id:"mock-transactions",level:2},{value:"L1-&gt;L2",id:"l1-l2",level:3},{value:"L2-&gt;L1",id:"l2-l1",level:3}];function l(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",strong:"strong",ul:"ul",...(0,r.R)(),...e.components};return(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(n.h1,{id:"l1-l2-interaction-via-postman",children:"L1-L2 interaction via Postman"}),"\n",(0,t.jsxs)(n.p,{children:["Postman is a Starknet utility that allows testing L1-L2 interaction. Ensure you have an L1 node and a Devnet (L2 node) running, ",(0,t.jsx)(n.a,{href:"#load",children:"load"})," a messaging contract, and ",(0,t.jsx)(n.a,{href:"#flush",children:"flush"})," the queue when needed. You can use ",(0,t.jsx)(n.a,{href:"https://github.com/0xSpaceShard/starknet-hardhat-plugin",children:(0,t.jsx)(n.strong,{children:(0,t.jsx)(n.code,{children:"starknet-hardhat-plugin"})})})," to perform these actions, as witnessed in ",(0,t.jsx)(n.a,{href:"https://github.com/0xSpaceShard/starknet-hardhat-example/blob/master/test/postman.test.ts",children:(0,t.jsx)(n.strong,{children:"this example"})}),", or directly send requests to the endpoints specified below."]}),"\n",(0,t.jsx)(n.h2,{id:"load",children:"Load"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"POST /postman/load_l1_messaging_contract\n"})}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-js",children:'{\n  "networkUrl": "http://localhost:8545",\n  "address": "0x123...def"\n}\n'})}),"\n",(0,t.jsxs)(n.p,{children:["Loads a ",(0,t.jsx)(n.code,{children:"MockStarknetMessaging"})," contract. The ",(0,t.jsx)(n.code,{children:"address"})," parameter is optional; if provided, the ",(0,t.jsx)(n.code,{children:"MockStarknetMessaging"})," contract will be fetched from that address, otherwise a new one will be deployed."]}),"\n",(0,t.jsxs)(n.p,{children:[(0,t.jsx)(n.code,{children:"networkUrl"})," is the URL of the JSON-RPC API of the L1 node you've run locally or that already exists; possibilities include, and are not limited to:"]}),"\n",(0,t.jsxs)(n.ul,{children:["\n",(0,t.jsx)(n.li,{children:(0,t.jsx)(n.a,{href:"https://github.com/foundry-rs/foundry/tree/master/crates/anvil",children:(0,t.jsx)(n.strong,{children:"Anvil"})})}),"\n",(0,t.jsx)(n.li,{children:(0,t.jsx)(n.a,{href:"https://sepolia.etherscan.io/",children:(0,t.jsx)(n.strong,{children:"Sepolia testnet"})})}),"\n",(0,t.jsx)(n.li,{children:(0,t.jsx)(n.a,{href:"https://www.npmjs.com/package/ganache",children:(0,t.jsx)(n.strong,{children:"Ganache"})})}),"\n",(0,t.jsx)(n.li,{children:(0,t.jsx)(n.a,{href:"https://github.com/ethereum/go-ethereum#docker-quick-start",children:(0,t.jsx)(n.strong,{children:"Geth"})})}),"\n",(0,t.jsx)(n.li,{children:(0,t.jsx)(n.a,{href:"https://hardhat.org/hardhat-network/#running-stand-alone-in-order-to-support-wallets-and-other-software",children:(0,t.jsx)(n.strong,{children:"Hardhat node"})})}),"\n"]}),"\n",(0,t.jsx)(n.h2,{id:"flush",children:"Flush"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"POST /postman/flush\n"})}),"\n",(0,t.jsxs)(n.p,{children:["Goes through the newly enqueued messages, sending them from L1 to L2 and from L2 to L1. Requires no body. Optionally, set the ",(0,t.jsx)(n.code,{children:"dry_run"})," specifier to just see the result of flushing, without actually triggering it:"]}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"POST /postman/flush\n"})}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-js",children:'{ "dry_run": true }\n'})}),"\n",(0,t.jsxs)(n.p,{children:["A running L1 node is required if ",(0,t.jsx)(n.code,{children:"dry_run"})," is not set."]}),"\n",(0,t.jsx)(n.h2,{id:"disclaimer",children:"Disclaimer"}),"\n",(0,t.jsxs)(n.p,{children:["This method of L1-L2 communication testing differs from how Starknet mainnet and testnets work. Taking ",(0,t.jsx)(n.a,{href:"https://github.com/MikeSpa/starknet-test/blob/6a68d033cd7ddb5df937154f860f1c06174e6860/L1L2Example.sol#L46",children:(0,t.jsx)(n.strong,{children:"L1L2Example.sol"})})," (originally from Starknet documentation, no longer available there):"]}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-solidity",children:"constructor(IStarknetCore starknetCore_) public {\n    starknetCore = starknetCore_;\n}\n"})}),"\n",(0,t.jsxs)(n.p,{children:["The constructor takes an ",(0,t.jsx)(n.code,{children:"IStarknetCore"})," contract as argument, however for Devnet's L1-L2 communication testing, this has to be replaced with the logic in ",(0,t.jsx)(n.a,{href:"https://github.com/starkware-libs/cairo-lang/blob/master/src/starkware/starknet/testing/MockStarknetMessaging.sol",children:(0,t.jsx)(n.strong,{children:"MockStarknetMessaging.sol"})}),":"]}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-solidity",children:"constructor(MockStarknetMessaging mockStarknetMessaging_) public {\n    starknetCore = mockStarknetMessaging_;\n}\n"})}),"\n",(0,t.jsx)(n.h2,{id:"mock-transactions",children:"Mock transactions"}),"\n",(0,t.jsx)(n.h3,{id:"l1-l2",children:"L1->L2"}),"\n",(0,t.jsxs)(n.p,{children:["Sending mock transactions from L1 to L2 without the need for running L1. Deployed L2 contract address ",(0,t.jsx)(n.code,{children:"l2_contract_address"})," and ",(0,t.jsx)(n.code,{children:"entry_point_selector"})," must be valid otherwise new block will not be created."]}),"\n",(0,t.jsxs)(n.p,{children:["Normally ",(0,t.jsx)(n.code,{children:"nonce"})," is calculated by L1 StarknetContract and it's used in L1 and L2. In this case, we need to provide it manually."]}),"\n",(0,t.jsxs)(n.p,{children:["A running L1 node is ",(0,t.jsx)(n.strong,{children:"not"})," required for this operation."]}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"POST /postman/send_message_to_l2\n"})}),"\n",(0,t.jsx)(n.p,{children:"Request:"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-js",children:'{\n    "l2_contract_address": "0x00285ddb7e5c777b310d806b9b2a0f7c7ba0a41f12b420219209d97a3b7f25b2",\n    "entry_point_selector": "0xC73F681176FC7B3F9693986FD7B14581E8D540519E27400E88B8713932BE01",\n    "l1_contract_address": "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",\n    "payload": [\n      "0x1",\n      "0x2"\n    ],\n    "paid_fee_on_l1": "0x123456abcdef"\n    "nonce":"0x0"\n}\n'})}),"\n",(0,t.jsx)(n.p,{children:"Response:"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-js",children:'{ "transaction_hash": "0x0548c761a9fd5512782998b2da6f44c42bf78fb88c3794eea330a91c9abb10bb" }\n'})}),"\n",(0,t.jsx)(n.h3,{id:"l2-l1",children:"L2->L1"}),"\n",(0,t.jsxs)(n.p,{children:["Sending mock transactions from L2 to L1.\nDeployed L2 contract address ",(0,t.jsx)(n.code,{children:"l2_contract_address"})," and ",(0,t.jsx)(n.code,{children:"l1_contract_address"})," must be valid."]}),"\n",(0,t.jsx)(n.p,{children:"A running L1 node is required for this operation."}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{children:"POST /postman/consume_message_from_l2\n"})}),"\n",(0,t.jsx)(n.p,{children:"Request:"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-js",children:'{\n    "l2_contract_address": "0x00285ddb7e5c777b310d806b9b2a0f7c7ba0a41f12b420219209d97a3b7f25b2",\n    "l1_contract_address": "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",\n    "payload": ["0x0", "0x1", "0x3e8"],\n}\n'})}),"\n",(0,t.jsx)(n.p,{children:"Response:"}),"\n",(0,t.jsx)(n.pre,{children:(0,t.jsx)(n.code,{className:"language-js",children:'{"message_hash": "0xae14f241131b524ac8d043d9cb4934253ac5c5589afef19f0d761816a9c7e26d"}\n'})})]})}function h(e={}){const{wrapper:n}={...(0,r.R)(),...e.components};return n?(0,t.jsx)(n,{...e,children:(0,t.jsx)(l,{...e})}):l(e)}},8453:(e,n,s)=>{s.d(n,{R:()=>o,x:()=>i});var t=s(6540);const r={},a=t.createContext(r);function o(e){const n=t.useContext(a);return t.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function i(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:o(e.components),t.createElement(a.Provider,{value:n},e.children)}}}]);
"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[2604],{4369:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>i,default:()=>h,frontMatter:()=>s,metadata:()=>r,toc:()=>a});var o=t(4848),c=t(8453);const s={},i="Blocks",r={id:"blocks",title:"Blocks",description:"By default, Devnet starts with a genesis block labelled with number zero. In forking mode, the genesis block number is equal to the forked block number plus one.",source:"@site/versioned_docs/version-0.1.2/blocks.md",sourceDirName:".",slug:"/blocks",permalink:"/starknet-devnet-rs/docs/blocks",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet-rs/blob/master/website/versioned_docs/version-0.1.2/blocks.md",tags:[],version:"0.1.2",frontMatter:{},sidebar:"docSidebar",previous:{title:"Account balance",permalink:"/starknet-devnet-rs/docs/balance"},next:{title:"Dump, load, restart",permalink:"/starknet-devnet-rs/docs/dump-load-restart"}},l={},a=[{value:"Creating blocks on transaction",id:"creating-blocks-on-transaction",level:2},{value:"Creating blocks on demand",id:"creating-blocks-on-demand",level:2},{value:"Automatic periodic block creation",id:"automatic-periodic-block-creation",level:2},{value:"Request new block creation",id:"request-new-block-creation",level:2},{value:"Abort blocks",id:"abort-blocks",level:2},{value:"Example",id:"example",level:3},{value:"Limitations",id:"limitations",level:3},{value:"Request and response",id:"request-and-response",level:3}];function d(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,c.R)(),...e.components};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(n.h1,{id:"blocks",children:"Blocks"}),"\n",(0,o.jsxs)(n.p,{children:["By default, Devnet starts with a genesis block labelled with number zero. In ",(0,o.jsx)(n.a,{href:"./forking",children:"forking mode"}),", the genesis block number is equal to the forked block number plus one."]}),"\n",(0,o.jsx)(n.h2,{id:"creating-blocks-on-transaction",children:"Creating blocks on transaction"}),"\n",(0,o.jsxs)(n.p,{children:["If you start Devnet with ",(0,o.jsx)(n.code,{children:"--block-generation-on transaction"}),", a new block is generated with each new transaction. This is the default block generation regime. This mode also supports ",(0,o.jsx)(n.a,{href:"#request-new-block-creation",children:"empty block creation"}),"."]}),"\n",(0,o.jsx)(n.h2,{id:"creating-blocks-on-demand",children:"Creating blocks on demand"}),"\n",(0,o.jsxs)(n.p,{children:["If you start Devnet with the ",(0,o.jsx)(n.code,{children:"--block-generation-on demand"})," CLI option, you will enable the possibility to store more than one transaction in the pending block (targetable via block tag ",(0,o.jsx)(n.code,{children:'"pending"'}),")."]}),"\n",(0,o.jsxs)(n.p,{children:["Once you've added the desired transactions into the pending block, you can ",(0,o.jsx)(n.a,{href:"#request-new-block-creation",children:"request new block creation"}),". This will convert the pending block to the latest block (targetable via block tag ",(0,o.jsx)(n.code,{children:'"latest"'}),"), giving it a block hash and a block number. All subsequent transactions will be stored in a new pending block."]}),"\n",(0,o.jsx)(n.p,{children:"In case of demanding block creation with no pending transactions, a new empty block will be generated."}),"\n",(0,o.jsx)(n.p,{children:"The creation of the genesis block is not affected by this feature."}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:"POST /create_block\n"})}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_createBlock"\n}\n'})}),"\n",(0,o.jsx)(n.p,{children:"Response:"}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:"{'block_hash': '0x115e1b390cafa7942b6ab141ab85040defe7dee9bef3bc31d8b5b3d01cc9c67'}\n"})}),"\n",(0,o.jsx)(n.h2,{id:"automatic-periodic-block-creation",children:"Automatic periodic block creation"}),"\n",(0,o.jsxs)(n.p,{children:["If started with the ",(0,o.jsx)(n.code,{children:"--block-generation-on <INTERVAL>"})," CLI option, Devnet will behave as in ",(0,o.jsxs)(n.a,{href:"#creating-blocks-on-demand",children:[(0,o.jsx)(n.code,{children:"demand"})," mode"]}),", but new blocks will be mined automatically every ",(0,o.jsx)(n.code,{children:"<INTERVAL>"})," seconds. Consider this example of spawning Devnet at moment ",(0,o.jsx)(n.code,{children:"t"}),":"]}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{className:"language-bash",children:"# t\n$ starknet-devnet --block-generation-on 10\n\n# t + 1s\n# user: send tx1\n\n# t + 4s\n# user: send tx2\n\n# t + 10s\n# Devnet: block automatically generated, contains tx1 and tx2\n\n# t + 12s\n# user: send tx3\n\n# t + 14s\n# user: invoke empty block creation\n# Devnet: generated block contains tx3\n\n# t + 20s\n# Devnet: block automatically generated, contains no txs (manual creation did not restart the counter)\n"})}),"\n",(0,o.jsx)(n.h2,{id:"request-new-block-creation",children:"Request new block creation"}),"\n",(0,o.jsxs)(n.p,{children:["To request the creation of a new block, ",(0,o.jsx)(n.code,{children:"POST"})," a request with no body to ",(0,o.jsx)(n.code,{children:"/create_block"})," or send:"]}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_createBlock"\n}\n'})}),"\n",(0,o.jsx)(n.p,{children:"Response:"}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'{"block_hash": "0x115e1b390cafa7942b6ab141ab85040defe7dee9bef3bc31d8b5b3d01cc9c67"}\n'})}),"\n",(0,o.jsx)(n.p,{children:"The newly created block will contain all pending transactions, if any, since the last block creation."}),"\n",(0,o.jsx)(n.h2,{id:"abort-blocks",children:"Abort blocks"}),"\n",(0,o.jsxs)(n.p,{children:["This functionality allows simulating block abortion that can occur on mainnet. It is supported in the ",(0,o.jsx)(n.code,{children:"--state-archive-capacity full"})," mode."]}),"\n",(0,o.jsx)(n.p,{children:"You can abort blocks and revert transactions from the specified block to the currently latest block. Newly created blocks after the abortion will have accepted status and will continue with numbering where the last accepted block left off."}),"\n",(0,o.jsx)(n.p,{children:"The state of Devnet will be reverted to the state of the last accepted block."}),"\n",(0,o.jsx)(n.h3,{id:"example",children:"Example"}),"\n",(0,o.jsx)(n.p,{children:"Assume there are 3 accepted blocks numbered 1, 2 and 3. Upon receiving a request to abort blocks starting with block 2, the blocks numbered 2 and 3 are aborted and their transactions reverted. The state of network will be as it was in block 1. Once a new block is mined, it will be accepted and it will have number 2."}),"\n",(0,o.jsx)(n.h3,{id:"limitations",children:"Limitations"}),"\n",(0,o.jsx)(n.p,{children:"Aborted blocks can only be queried by block hash. Devnet does not support the abortion of:"}),"\n",(0,o.jsxs)(n.ul,{children:["\n",(0,o.jsx)(n.li,{children:"blocks in the forking origin (i.e. blocks mined before the forked block)"}),"\n",(0,o.jsx)(n.li,{children:"already aborted blocks"}),"\n",(0,o.jsx)(n.li,{children:"Devnet's genesis block"}),"\n"]}),"\n",(0,o.jsx)(n.h3,{id:"request-and-response",children:"Request and response"}),"\n",(0,o.jsx)(n.p,{children:"To abort, send one of the following:"}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'POST /abort_blocks\n{\n    "starting_block_hash": BLOCK_HASH\n}\n'})}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_abortBlocks",\n    "params": {\n        "starting_block_hash": BLOCK_HASH\n    }\n}\n'})}),"\n",(0,o.jsx)(n.p,{children:"Response:"}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'{\n    "aborted": [BLOCK_HASH_0, BLOCK_HASH_1, ...]\n}\n'})})]})}function h(e={}){const{wrapper:n}={...(0,c.R)(),...e.components};return n?(0,o.jsx)(n,{...e,children:(0,o.jsx)(d,{...e})}):d(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>i,x:()=>r});var o=t(6540);const c={},s=o.createContext(c);function i(e){const n=o.useContext(s);return o.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function r(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(c):e.components||c:i(e.components),o.createElement(s.Provider,{value:n},e.children)}}}]);
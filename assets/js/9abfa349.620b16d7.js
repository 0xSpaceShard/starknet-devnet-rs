"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[2408],{8521:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>r,default:()=>b,frontMatter:()=>s,metadata:()=>a,toc:()=>i});var o=t(4848),c=t(8453);const s={},r="Blocks",a={id:"blocks",title:"Blocks",description:"Devnet starts with a genesis block (with a block number equal to 0). In forking mode, the genesis block number will be equal to the forked block number plus one.",source:"@site/versioned_docs/version-0.0.7/blocks.md",sourceDirName:".",slug:"/blocks",permalink:"/starknet-devnet-rs/docs/0.0.7/blocks",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet-rs/blob/master/website/versioned_docs/version-0.0.7/blocks.md",tags:[],version:"0.0.7",frontMatter:{},sidebar:"docSidebar",previous:{title:"Account balance",permalink:"/starknet-devnet-rs/docs/0.0.7/balance"},next:{title:"Dump, load, restart",permalink:"/starknet-devnet-rs/docs/0.0.7/dump-load-restart"}},l={},i=[{value:"Creating blocks on demand",id:"creating-blocks-on-demand",level:2},{value:"Create an empty block",id:"create-an-empty-block",level:2},{value:"Abort blocks",id:"abort-blocks",level:2}];function d(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",p:"p",pre:"pre",...(0,c.R)(),...e.components};return(0,o.jsxs)(o.Fragment,{children:[(0,o.jsx)(n.h1,{id:"blocks",children:"Blocks"}),"\n",(0,o.jsx)(n.p,{children:"Devnet starts with a genesis block (with a block number equal to 0). In forking mode, the genesis block number will be equal to the forked block number plus one."}),"\n",(0,o.jsxs)(n.p,{children:["A new block is generated based on the pending block, once a new block is generated the pending block is restarted. By default, a new block is generated with each new transaction, but you can also ",(0,o.jsx)(n.a,{href:"#create-an-empty-block",children:"create an empty block by yourself"}),"."]}),"\n",(0,o.jsx)(n.h2,{id:"creating-blocks-on-demand",children:"Creating blocks on demand"}),"\n",(0,o.jsxs)(n.p,{children:["If you start Devnet with the ",(0,o.jsx)(n.code,{children:"--blocks-on-demand"})," CLI option, you will enable the possibility to store more than one transaction in the pending block (targetable via block tag ",(0,o.jsx)(n.code,{children:'"pending"'}),")."]}),"\n",(0,o.jsxs)(n.p,{children:["Once you've added the desired transactions into the pending block, you can send a ",(0,o.jsx)(n.code,{children:"POST"})," request to ",(0,o.jsx)(n.code,{children:"/create_block"}),". This will convert the pending block to the latest block (targetable via block tag ",(0,o.jsx)(n.code,{children:'"latest"'}),"), giving it a block hash and a block number. All subsequent transactions will be stored in a new pending block."]}),"\n",(0,o.jsx)(n.p,{children:"In case of demanding block creation with no pending transactions, a new empty block will be generated."}),"\n",(0,o.jsx)(n.p,{children:"The creation of the genesis block is not affected by this feature."}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:"POST /create_block\n"})}),"\n",(0,o.jsx)(n.p,{children:"Response:"}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:"{'block_hash': '0x115e1b390cafa7942b6ab141ab85040defe7dee9bef3bc31d8b5b3d01cc9c67'}\n"})}),"\n",(0,o.jsx)(n.h2,{id:"create-an-empty-block",children:"Create an empty block"}),"\n",(0,o.jsxs)(n.p,{children:["To create an empty block without transactions, ",(0,o.jsx)(n.code,{children:"POST"})," a request to ",(0,o.jsx)(n.code,{children:"/create_block"}),":"]}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:"POST /create_block\n"})}),"\n",(0,o.jsx)(n.p,{children:"Response:"}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'{"block_hash": "0x115e1b390cafa7942b6ab141ab85040defe7dee9bef3bc31d8b5b3d01cc9c67"}\n'})}),"\n",(0,o.jsx)(n.h2,{id:"abort-blocks",children:"Abort blocks"}),"\n",(0,o.jsx)(n.p,{children:"This functionality allows simulating block abortion that can occur on mainnet."}),"\n",(0,o.jsx)(n.p,{children:"You can abort blocks and revert transactions from the specified block to the currently latest block. Newly created blocks after the abortion will have accepted status and will continue with numbering where the last accepted block left off."}),"\n",(0,o.jsx)(n.p,{children:"The state of Devnet will be reverted to the state of the last accepted block."}),"\n",(0,o.jsx)(n.p,{children:"E.g. assume there are 3 accepted blocks numbered 1, 2 and 3. Upon receiving a request to abort blocks starting with block 2, the blocks numbered 2 and 3 are aborted and their transactions reverted. The state of network will be as it was in block 1. Once a new block is mined, it will be accepted and it will have number 2."}),"\n",(0,o.jsx)(n.p,{children:"Aborted blocks can only be queried by block hash. Aborting the blocks in forking origin and already aborted blocks is not supported and results in an error."}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'POST /abort_blocks\n{\n    "starting_block_hash": BLOCK_HASH\n}\n'})}),"\n",(0,o.jsx)(n.p,{children:"Response:"}),"\n",(0,o.jsx)(n.pre,{children:(0,o.jsx)(n.code,{children:'{\n    "aborted": [BLOCK_HASH_0, BLOCK_HASH_1, ...]\n}\n'})})]})}function b(e={}){const{wrapper:n}={...(0,c.R)(),...e.components};return n?(0,o.jsx)(n,{...e,children:(0,o.jsx)(d,{...e})}):d(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>r,x:()=>a});var o=t(6540);const c={},s=o.createContext(c);function r(e){const n=o.useContext(s);return o.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function a(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(c):e.components||c:r(e.components),o.createElement(s.Provider,{value:n},e.children)}}}]);
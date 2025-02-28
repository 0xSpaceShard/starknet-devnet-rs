"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[8398],{6892:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>a,contentTitle:()=>i,default:()=>h,frontMatter:()=>r,metadata:()=>o,toc:()=>l});var s=t(4848),d=t(8453);const r={},i="Dump, load, restart",o={id:"dump-load-restart",title:"Dump, load, restart",description:"Dumping",source:"@site/versioned_docs/version-0.2.1/dump-load-restart.md",sourceDirName:".",slug:"/dump-load-restart",permalink:"/starknet-devnet/docs/0.2.1/dump-load-restart",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet/blob/master/website/versioned_docs/version-0.2.1/dump-load-restart.md",tags:[],version:"0.2.1",frontMatter:{},sidebar:"docSidebar",previous:{title:"Blocks",permalink:"/starknet-devnet/docs/0.2.1/blocks"},next:{title:"Examples",permalink:"/starknet-devnet/docs/0.2.1/examples"}},a={},l=[{value:"Dumping",id:"dumping",level:2},{value:"Dumping on request",id:"dumping-on-request",level:3},{value:"Loading",id:"loading",level:2},{value:"Loading disclaimer",id:"loading-disclaimer",level:3},{value:"Restarting",id:"restarting",level:2},{value:"Restarting and L1-L2 messaging",id:"restarting-and-l1-l2-messaging",level:3},{value:"Docker",id:"docker",level:2}];function c(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,d.R)(),...e.components};return(0,s.jsxs)(s.Fragment,{children:[(0,s.jsx)(n.h1,{id:"dump-load-restart",children:"Dump, load, restart"}),"\n",(0,s.jsx)(n.h2,{id:"dumping",children:"Dumping"}),"\n",(0,s.jsx)(n.p,{children:"To preserve your Devnet instance for future use, these are the options:"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"Dumping on exit (handles Ctrl+C, i.e. SIGINT; doesn't handle SIGKILL):"}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:"$ starknet-devnet --dump-on exit --dump-path <PATH>\n"})}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"Dumping after each block:"}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:"$ starknet-devnet --dump-on block --dump-path <PATH>\n"})}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["Dumping on request, which requires providing ",(0,s.jsx)(n.code,{children:"--dump-on request"})," on startup. You can also dump on request if you specified any of the other ",(0,s.jsx)(n.code,{children:"--dump-on"})," modes."]}),"\n"]}),"\n",(0,s.jsx)(n.h3,{id:"dumping-on-request",children:"Dumping on request"}),"\n",(0,s.jsxs)(n.p,{children:["You can request dumping by sending ",(0,s.jsx)(n.code,{children:"POST"})," to ",(0,s.jsx)(n.code,{children:"/dump"})," or via JSON-RPC. An optional file path can be provided in the request or on startup via ",(0,s.jsx)(n.code,{children:"--dump-path <FILE>"})," (the HTTP request parameter takes precedence). If no dumping path is specified, the dump is included in the response body. This means that if you request dumping via ",(0,s.jsx)(n.a,{href:"https://curl.se/",children:(0,s.jsx)(n.code,{children:"curl"})}),", it will be printed to STDOUT, which you can then redirect to a destination of your choice."]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:"$ starknet-devnet --dump-on <MODE> [--dump-path <FILE>]\n"})}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"No body parameters:"}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:"POST /dump\n"})}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_dump"\n}\n'})}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsx)(n.li,{children:"With a custom path:"}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:'POST /dump\n{\n  // optional; defaults to the path specified via CLI if defined\n  "path": <PATH>\n}\n'})}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_dump",\n    "params": {\n        // optional; defaults to the path specified via CLI if defined\n        "path": <PATH>\n    }\n}\n'})}),"\n",(0,s.jsx)(n.h2,{id:"loading",children:"Loading"}),"\n",(0,s.jsx)(n.p,{children:"To load a preserved Devnet instance, the options are:"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["Loading on startup (note the argument name is not ",(0,s.jsx)(n.code,{children:"--load-path"})," as it was in Devnet-py):"]}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:"$ starknet-devnet --dump-path <PATH>\n"})}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["Loading on request, which replaces the current state with the one in the provided file. It can be done by sending ",(0,s.jsx)(n.code,{children:"POST"})," to ",(0,s.jsx)(n.code,{children:"/load"})," or via JSON-RPC:"]}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:'POST /load\n{ "path": <PATH> }\n'})}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_load",\n    "params": {\n        "path": <PATH>\n    }\n}\n'})}),"\n",(0,s.jsx)(n.h3,{id:"loading-disclaimer",children:"Loading disclaimer"}),"\n",(0,s.jsxs)(n.p,{children:["Currently, dumping produces a list of reproducible Devnet actions (state-changing requests and transactions). Conversely, loading is implemented as the re-execution of transactions from a dump. This means that timestamps of ",(0,s.jsx)(n.code,{children:"StarknetBlock"})," will be different on each load. This is due to the nature of Devnet's dependencies, which prevent Devnet's state from being serialized."]}),"\n",(0,s.jsx)(n.p,{children:"Dumping and loading are not guaranteed to work across versions. I.e. if you dumped one version of Devnet, do not expect it to be loadable with a different version."}),"\n",(0,s.jsxs)(n.p,{children:["If you dumped a Devnet utilizing one class for account predeployment (e.g. ",(0,s.jsx)(n.code,{children:"--account-class cairo0"}),"), you should use the same option when loading. The same applies for dumping a Devnet in ",(0,s.jsx)(n.code,{children:"--block-generation-on demand"})," mode."]}),"\n",(0,s.jsx)(n.h2,{id:"restarting",children:"Restarting"}),"\n",(0,s.jsxs)(n.p,{children:["Devnet can be restarted by making a ",(0,s.jsx)(n.code,{children:"POST /restart"})," request (no body required) or ",(0,s.jsx)(n.code,{children:"JSON-RPC"})," request with method name ",(0,s.jsx)(n.code,{children:"devnet_restart"}),". All of the deployed contracts (including predeployed), blocks and storage updates will be restarted to the original state, without the transactions and requests that may have been loaded from a dump file on startup."]}),"\n",(0,s.jsx)(n.h3,{id:"restarting-and-l1-l2-messaging",children:"Restarting and L1-L2 messaging"}),"\n",(0,s.jsxs)(n.p,{children:["If you're doing ",(0,s.jsx)(n.a,{href:"./postman",children:"L1-L2 message exchange"}),", restarting will by default not affect Devnet's connection with L1 nor the L1->L2 message queue. The effect that L1-L2 messages may have had on Devnet before restarting shall be reverted, including any L2 contracts used for messaging. Also, calling ",(0,s.jsx)(n.a,{href:"./postman#flush",children:(0,s.jsx)(n.code,{children:"flush"})})," will not have new messages to read until they are actually sent. If you wish to re-process the already-seen L1->L2 messages when you restart, make them accessible again by setting the ",(0,s.jsx)(n.code,{children:"restart_l1_to_l2_messaging"})," parameter shown below. If you set this flag:"]}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["you will need to ",(0,s.jsx)(n.a,{href:"./postman#load",children:"reload the L1-side messaging contract"})]}),"\n",(0,s.jsxs)(n.li,{children:["the L1->L2 messages won't be restarted in the sense of being deleted, but access to them shall be regained via ",(0,s.jsx)(n.a,{href:"./postman#flush",children:(0,s.jsx)(n.code,{children:"flush"})})]}),"\n",(0,s.jsx)(n.li,{children:"the L2->L1 message queue is restarted regardless of the flag"}),"\n"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_restart",\n    "params": {\n        // optional parameter, defaults to false\n        "restart_l1_to_l2_messaging": true | false\n    }\n}\n'})}),"\n",(0,s.jsx)(n.h2,{id:"docker",children:"Docker"}),"\n",(0,s.jsx)(n.p,{children:"To enable dumping and loading with dockerized Devnet, you must bind the container path to the path on your host machine."}),"\n",(0,s.jsx)(n.p,{children:"This example:"}),"\n",(0,s.jsxs)(n.ul,{children:["\n",(0,s.jsxs)(n.li,{children:["Relies on ",(0,s.jsx)(n.a,{href:"https://docs.docker.com/storage/bind-mounts/",children:"Docker bind mount"}),"; try ",(0,s.jsx)(n.a,{href:"https://docs.docker.com/storage/volumes/",children:"Docker volume"})," instead."]}),"\n",(0,s.jsxs)(n.li,{children:["Assumes that ",(0,s.jsx)(n.code,{children:"/path/to/dumpdir"})," exists. If unsure, use absolute paths."]}),"\n",(0,s.jsxs)(n.li,{children:["Assumes you are listening on ",(0,s.jsx)(n.code,{children:"127.0.0.1:5050"}),"."]}),"\n"]}),"\n",(0,s.jsxs)(n.p,{children:["If there is ",(0,s.jsx)(n.code,{children:"mydump"})," inside ",(0,s.jsx)(n.code,{children:"/path/to/dumpdir"}),", you can load it with:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:"docker run \\\n  -p 127.0.0.1:5050:5050 \\\n  --mount type=bind,source=/path/to/dumpdir,target=/path/to/dumpdir \\\n  shardlabs/starknet-devnet-rs \\\n  --dump-path /path/to/dumpdir/mydump\n"})}),"\n",(0,s.jsxs)(n.p,{children:["To dump to ",(0,s.jsx)(n.code,{children:"/path/to/dumpdir/mydump"})," on Devnet shutdown, run:"]}),"\n",(0,s.jsx)(n.pre,{children:(0,s.jsx)(n.code,{children:"docker run \\\n  -p 127.0.0.1:5050:5050 \\\n  --mount type=bind,source=/path/to/dumpdir,target=/path/to/dumpdir \\\n  shardlabs/starknet-devnet-rs \\\n  --dump-on exit --dump-path /path/to/dumpdir/mydump\n"})})]})}function h(e={}){const{wrapper:n}={...(0,d.R)(),...e.components};return n?(0,s.jsx)(n,{...e,children:(0,s.jsx)(c,{...e})}):c(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>i,x:()=>o});var s=t(6540);const d={},r=s.createContext(d);function i(e){const n=s.useContext(r);return s.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function o(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(d):e.components||d:i(e.components),s.createElement(r.Provider,{value:n},e.children)}}}]);
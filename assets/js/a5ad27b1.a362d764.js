"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[5120],{6989:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>a,contentTitle:()=>o,default:()=>p,frontMatter:()=>s,metadata:()=>i,toc:()=>l});var d=t(4848),r=t(8453);const s={},o="Dump, load, restart",i={id:"dump-load-restart",title:"Dump, load, restart",description:"Dumping",source:"@site/docs/dump-load-restart.md",sourceDirName:".",slug:"/dump-load-restart",permalink:"/starknet-devnet-rs/docs/next/dump-load-restart",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet-rs/blob/master/website/docs/dump-load-restart.md",tags:[],version:"current",frontMatter:{},sidebar:"docSidebar",previous:{title:"Blocks",permalink:"/starknet-devnet-rs/docs/next/blocks"},next:{title:"Forking",permalink:"/starknet-devnet-rs/docs/next/forking"}},a={},l=[{value:"Dumping",id:"dumping",level:2},{value:"Loading",id:"loading",level:2},{value:"Loading disclaimer",id:"loading-disclaimer",level:3},{value:"Restarting",id:"restarting",level:2},{value:"Docker",id:"docker",level:2}];function c(e){const n={a:"a",code:"code",h1:"h1",h2:"h2",h3:"h3",li:"li",p:"p",pre:"pre",ul:"ul",...(0,r.R)(),...e.components};return(0,d.jsxs)(d.Fragment,{children:[(0,d.jsx)(n.h1,{id:"dump-load-restart",children:"Dump, load, restart"}),"\n",(0,d.jsx)(n.h2,{id:"dumping",children:"Dumping"}),"\n",(0,d.jsx)(n.p,{children:"To preserve your Devnet instance for future use, these are the options:"}),"\n",(0,d.jsxs)(n.ul,{children:["\n",(0,d.jsx)(n.li,{children:"Dumping on exit (handles Ctrl+C, i.e. SIGINT; doesn't handle SIGKILL):"}),"\n"]}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:"$ starknet-devnet --dump-on exit --dump-path <PATH>\n"})}),"\n",(0,d.jsxs)(n.ul,{children:["\n",(0,d.jsx)(n.li,{children:"Dumping after each block:"}),"\n"]}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:"$ starknet-devnet --dump-on block --dump-path <PATH>\n"})}),"\n",(0,d.jsxs)(n.ul,{children:["\n",(0,d.jsxs)(n.li,{children:["Dumping on request requires providing --dump-on mode on the startup. Example usage in ",(0,d.jsx)(n.code,{children:"exit"})," mode (replace ",(0,d.jsx)(n.code,{children:"<HOST>"}),", ",(0,d.jsx)(n.code,{children:"<PORT>"})," and ",(0,d.jsx)(n.code,{children:"<PATH>"})," with your own):"]}),"\n"]}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:'$ starknet-devnet --dump-on exit --dump-path <PATH>\n$ curl -X POST http://<HOST>:<PORT>/dump -d \'{ "path": <PATH> }\' -H "Content-Type: application/json"\n'})}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_dump",\n    "params": {\n        "path": PATH\n    }\n}\n'})}),"\n",(0,d.jsx)(n.h2,{id:"loading",children:"Loading"}),"\n",(0,d.jsx)(n.p,{children:"To load a preserved Devnet instance, the options are:"}),"\n",(0,d.jsxs)(n.ul,{children:["\n",(0,d.jsxs)(n.li,{children:["Loading on startup (note the argument name is not ",(0,d.jsx)(n.code,{children:"--load-path"})," as it was in Devnet-py):"]}),"\n"]}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:"$ starknet-devnet --dump-path <PATH>\n"})}),"\n",(0,d.jsxs)(n.ul,{children:["\n",(0,d.jsx)(n.li,{children:"Loading on request:"}),"\n"]}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:'curl -X POST http://<HOST>:<PORT>/load -d \'{ "path": <PATH> }\' -H "Content-Type: application/json"\n'})}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_load",\n    "params": {\n        "path": PATH\n    }\n}\n'})}),"\n",(0,d.jsxs)(n.p,{children:["Currently, dumping produces a list of received transactions that is stored on disk. Conversely, loading is implemented as the re-execution of transactions from a dump. This means that timestamps of ",(0,d.jsx)(n.code,{children:"StarknetBlock"})," will be different on each load."]}),"\n",(0,d.jsx)(n.h3,{id:"loading-disclaimer",children:"Loading disclaimer"}),"\n",(0,d.jsx)(n.p,{children:"Dumping and loading are not guaranteed to work across versions. I.e. if you dumped one version of Devnet, do not expect it to be loadable with a different version."}),"\n",(0,d.jsxs)(n.p,{children:["If you dumped a Devnet utilizing one class for account predeployment (e.g. ",(0,d.jsx)(n.code,{children:"--account-class cairo0"}),"), you should use the same option when loading. The same applies for dumping a Devnet in ",(0,d.jsx)(n.code,{children:"--block-generation-on demand"})," mode."]}),"\n",(0,d.jsx)(n.h2,{id:"restarting",children:"Restarting"}),"\n",(0,d.jsxs)(n.p,{children:["Devnet can be restarted by making a ",(0,d.jsx)(n.code,{children:"POST /restart"})," request (no body required) or ",(0,d.jsx)(n.code,{children:"JSON-RPC"})," request with method name ",(0,d.jsx)(n.code,{children:"devnet_restart"}),". All of the deployed contracts (including predeployed), blocks and storage updates will be restarted to the original state, without the transactions and requests that may have been loaded from a dump file on startup."]}),"\n",(0,d.jsx)(n.h2,{id:"docker",children:"Docker"}),"\n",(0,d.jsx)(n.p,{children:"To enable dumping and loading with dockerized Devnet, you must bind the container path to the path on your host machine."}),"\n",(0,d.jsx)(n.p,{children:"This example:"}),"\n",(0,d.jsxs)(n.ul,{children:["\n",(0,d.jsxs)(n.li,{children:["Relies on ",(0,d.jsx)(n.a,{href:"https://docs.docker.com/storage/bind-mounts/",children:"Docker bind mount"}),"; try ",(0,d.jsx)(n.a,{href:"https://docs.docker.com/storage/volumes/",children:"Docker volume"})," instead."]}),"\n",(0,d.jsxs)(n.li,{children:["Assumes that ",(0,d.jsx)(n.code,{children:"/path/to/dumpdir"})," exists. If unsure, use absolute paths."]}),"\n",(0,d.jsxs)(n.li,{children:["Assumes you are listening on ",(0,d.jsx)(n.code,{children:"127.0.0.1:5050"}),"."]}),"\n"]}),"\n",(0,d.jsxs)(n.p,{children:["If there is ",(0,d.jsx)(n.code,{children:"mydump"})," inside ",(0,d.jsx)(n.code,{children:"/path/to/dumpdir"}),", you can load it with:"]}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:"docker run \\\n  -p 127.0.0.1:5050:5050 \\\n  --mount type=bind,source=/path/to/dumpdir,target=/path/to/dumpdir \\\n  shardlabs/starknet-devnet-rs \\\n  --dump-path /path/to/dumpdir/mydump\n"})}),"\n",(0,d.jsxs)(n.p,{children:["To dump to ",(0,d.jsx)(n.code,{children:"/path/to/dumpdir/mydump"})," on Devnet shutdown, run:"]}),"\n",(0,d.jsx)(n.pre,{children:(0,d.jsx)(n.code,{children:"docker run \\\n  -p 127.0.0.1:5050:5050 \\\n  --mount type=bind,source=/path/to/dumpdir,target=/path/to/dumpdir \\\n  shardlabs/starknet-devnet-rs \\\n  --dump-on exit --dump-path /path/to/dumpdir/mydump\n"})})]})}function p(e={}){const{wrapper:n}={...(0,r.R)(),...e.components};return n?(0,d.jsx)(n,{...e,children:(0,d.jsx)(c,{...e})}):c(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>o,x:()=>i});var d=t(6540);const r={},s=d.createContext(r);function o(e){const n=d.useContext(s);return d.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function i(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(r):e.components||r:o(e.components),d.createElement(s.Provider,{value:n},e.children)}}}]);
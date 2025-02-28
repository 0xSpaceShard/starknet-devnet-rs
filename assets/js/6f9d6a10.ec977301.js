"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[7104],{123:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>c,contentTitle:()=>o,default:()=>h,frontMatter:()=>s,metadata:()=>d,toc:()=>l});var r=t(4848),i=t(8453);const s={},o="Server config",d={id:"server-config",title:"Server config",description:"To read generally about ways to configure your Devnet instance, check out the CLI section.",source:"@site/versioned_docs/version-0.2.1/server-config.md",sourceDirName:".",slug:"/server-config",permalink:"/starknet-devnet/docs/0.2.1/server-config",draft:!1,unlisted:!1,editUrl:"https://github.com/0xSpaceShard/starknet-devnet/blob/master/website/versioned_docs/version-0.2.1/server-config.md",tags:[],version:"0.2.1",frontMatter:{},sidebar:"docSidebar",previous:{title:"Restrictive mode",permalink:"/starknet-devnet/docs/0.2.1/restrictive"},next:{title:"Starknet time",permalink:"/starknet-devnet/docs/0.2.1/starknet-time"}},c={},l=[{value:"Host and port",id:"host-and-port",level:2},{value:"Logging",id:"logging",level:2},{value:"Timeout",id:"timeout",level:2},{value:"Request body size limit",id:"request-body-size-limit",level:2},{value:"API",id:"api",level:2}];function a(e){const n={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",p:"p",pre:"pre",...(0,i.R)(),...e.components};return(0,r.jsxs)(r.Fragment,{children:[(0,r.jsx)(n.h1,{id:"server-config",children:"Server config"}),"\n",(0,r.jsxs)(n.p,{children:["To read generally about ways to configure your Devnet instance, check out the ",(0,r.jsx)(n.a,{href:"/starknet-devnet/docs/0.2.1/running/cli",children:"CLI section"}),"."]}),"\n",(0,r.jsx)(n.h2,{id:"host-and-port",children:"Host and port"}),"\n",(0,r.jsxs)(n.p,{children:["Specify the host and the port used by the server with ",(0,r.jsx)(n.code,{children:"--host <ADDRESS>"})," and ",(0,r.jsx)(n.code,{children:"--port <NUMBER>"})," CLI arguments. If running with Docker, check out the ",(0,r.jsx)(n.a,{href:"./running/docker#container-port-publishing",children:"port publishing docs"}),"."]}),"\n",(0,r.jsx)(n.h2,{id:"logging",children:"Logging"}),"\n",(0,r.jsxs)(n.p,{children:["By default, the logging level is ",(0,r.jsx)(n.code,{children:"INFO"}),", but this can be changed via the ",(0,r.jsx)(n.code,{children:"RUST_LOG"})," environment variable."]}),"\n",(0,r.jsxs)(n.p,{children:["All logging levels: ",(0,r.jsx)(n.code,{children:"TRACE"}),", ",(0,r.jsx)(n.code,{children:"DEBUG"}),", ",(0,r.jsx)(n.code,{children:"INFO"}),", ",(0,r.jsx)(n.code,{children:"WARN"}),", ",(0,r.jsx)(n.code,{children:"ERROR"})]}),"\n",(0,r.jsx)(n.p,{children:"To specify the logging level and run Devnet on the same line:"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:"$ RUST_LOG=<LEVEL> starknet-devnet\n"})}),"\n",(0,r.jsx)(n.p,{children:"or if using dockerized Devnet:"}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:"$ docker run -e RUST_LOG=<LEVEL> shardlabs/starknet-devnet-rs\n"})}),"\n",(0,r.jsxs)(n.p,{children:["By default, logging of request and response data is turned off.\nTo see the request and/or response body, additional levels can be specified via the ",(0,r.jsx)(n.code,{children:"RUST_LOG"})," environment variable: ",(0,r.jsx)(n.code,{children:"REQUEST"})," for request body, ",(0,r.jsx)(n.code,{children:"RESPONSE"})," for response body."]}),"\n",(0,r.jsxs)(n.admonition,{type:"note",children:[(0,r.jsxs)(n.p,{children:["Logging request and response requires at least logging level ",(0,r.jsx)(n.code,{children:"INFO"}),"."]}),(0,r.jsxs)(n.p,{children:["For example, the following two commands will log request and response data with log level ",(0,r.jsx)(n.code,{children:"INFO"}),"."]}),(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:'$ RUST_LOG="REQUEST,RESPONSE" starknet-devnet\n'})}),(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:'$ RUST_LOG="REQUEST,RESPONSE,INFO" starknet-devnet\n'})})]}),"\n",(0,r.jsx)(n.h2,{id:"timeout",children:"Timeout"}),"\n",(0,r.jsx)(n.p,{children:"Specify the maximum amount of time an HTTP request can be served. This makes it possible to deploy and manage large contracts that take longer to execute."}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:"$ starknet-devnet --timeout <SECONDS>\n"})}),"\n",(0,r.jsx)(n.h2,{id:"request-body-size-limit",children:"Request body size limit"}),"\n",(0,r.jsx)(n.p,{children:"Specify the maximum size of an incoming HTTP request body. This makes it possible to deploy and manage large contracts that take up more space."}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:"$ starknet-devnet --request-body-size-limit <BYTES>\n"})}),"\n",(0,r.jsx)(n.h2,{id:"api",children:"API"}),"\n",(0,r.jsxs)(n.p,{children:["Retrieve the server config by sending a ",(0,r.jsx)(n.code,{children:"GET"})," request to ",(0,r.jsx)(n.code,{children:"/config"})," or ",(0,r.jsx)(n.code,{children:"JSON-RPC"})," request with method name ",(0,r.jsx)(n.code,{children:"devnet_getConfig"})," and extracting its ",(0,r.jsx)(n.code,{children:"server_config"})," property."]}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:"$ curl localhost:5050/config | jq .server_config\n"})}),"\n",(0,r.jsx)(n.pre,{children:(0,r.jsx)(n.code,{children:'JSON-RPC\n{\n    "jsonrpc": "2.0",\n    "id": "1",\n    "method": "devnet_getConfig"\n}\n'})})]})}function h(e={}){const{wrapper:n}={...(0,i.R)(),...e.components};return n?(0,r.jsx)(n,{...e,children:(0,r.jsx)(a,{...e})}):a(e)}},8453:(e,n,t)=>{t.d(n,{R:()=>o,x:()=>d});var r=t(6540);const i={},s=r.createContext(i);function o(e){const n=r.useContext(s);return r.useMemo((function(){return"function"==typeof e?e(n):{...n,...e}}),[n,e])}function d(e){let n;return n=e.disableParentContext?"function"==typeof e.components?e.components(i):e.components||i:o(e.components),r.createElement(s.Provider,{value:n},e.children)}}}]);
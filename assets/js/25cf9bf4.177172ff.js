"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[4512],{3206:e=>{e.exports=JSON.parse('{"version":{"pluginId":"default","version":"0.2.3","label":"0.2.3","banner":"unmaintained","badge":true,"noIndex":false,"className":"docs-version-0.2.3","isLast":false,"docsSidebars":{"docSidebar":[{"type":"link","label":"Intro","href":"/starknet-devnet-rs/docs/0.2.3/intro","docId":"intro","unlisted":false},{"type":"category","label":"Running","collapsible":true,"collapsed":true,"items":[{"type":"link","label":"Install and run","href":"/starknet-devnet-rs/docs/0.2.3/running/install","docId":"running/install","unlisted":false},{"type":"link","label":"Run with Docker","href":"/starknet-devnet-rs/docs/0.2.3/running/docker","docId":"running/docker","unlisted":false},{"type":"link","label":"CLI options","href":"/starknet-devnet-rs/docs/0.2.3/running/cli","docId":"running/cli","unlisted":false}],"href":"/starknet-devnet-rs/docs/0.2.3/category/running"},{"type":"link","label":"API","href":"/starknet-devnet-rs/docs/0.2.3/api","docId":"api","unlisted":false},{"type":"link","label":"Account impersonation","href":"/starknet-devnet-rs/docs/0.2.3/account-impersonation","docId":"account-impersonation","unlisted":false},{"type":"link","label":"Account balance","href":"/starknet-devnet-rs/docs/0.2.3/balance","docId":"balance","unlisted":false},{"type":"link","label":"Blocks","href":"/starknet-devnet-rs/docs/0.2.3/blocks","docId":"blocks","unlisted":false},{"type":"link","label":"Dump, load, restart","href":"/starknet-devnet-rs/docs/0.2.3/dump-load-restart","docId":"dump-load-restart","unlisted":false},{"type":"link","label":"Examples","href":"/starknet-devnet-rs/docs/0.2.3/examples","docId":"examples","unlisted":false},{"type":"link","label":"Forking","href":"/starknet-devnet-rs/docs/0.2.3/forking","docId":"forking","unlisted":false},{"type":"link","label":"Gas price modification","href":"/starknet-devnet-rs/docs/0.2.3/gas","docId":"gas","unlisted":false},{"type":"link","label":"Historic state support","href":"/starknet-devnet-rs/docs/0.2.3/historic-state","docId":"historic-state","unlisted":false},{"type":"link","label":"Lite mode","href":"/starknet-devnet-rs/docs/0.2.3/lite","docId":"lite","unlisted":false},{"type":"link","label":"L1-L2 interaction via Postman","href":"/starknet-devnet-rs/docs/0.2.3/postman","docId":"postman","unlisted":false},{"type":"link","label":"Predeployed contracts","href":"/starknet-devnet-rs/docs/0.2.3/predeployed","docId":"predeployed","unlisted":false},{"type":"link","label":"Restrictive mode","href":"/starknet-devnet-rs/docs/0.2.3/restrictive","docId":"restrictive","unlisted":false},{"type":"link","label":"Server config","href":"/starknet-devnet-rs/docs/0.2.3/server-config","docId":"server-config","unlisted":false},{"type":"link","label":"Starknet time","href":"/starknet-devnet-rs/docs/0.2.3/starknet-time","docId":"starknet-time","unlisted":false}]},"docs":{"account-impersonation":{"id":"account-impersonation","title":"Account impersonation","description":"This page is about account impersonation. To read about account class selection and deployment, click here.","sidebar":"docSidebar"},"api":{"id":"api","title":"API","description":"JSON-RPC API","sidebar":"docSidebar"},"balance":{"id":"balance","title":"Account balance","description":"Other than using prefunded predeployed accounts, you can also add funds to an account that you deployed yourself.","sidebar":"docSidebar"},"blocks":{"id":"blocks","title":"Blocks","description":"Genesis block","sidebar":"docSidebar"},"dump-load-restart":{"id":"dump-load-restart","title":"Dump, load, restart","description":"Dumping","sidebar":"docSidebar"},"examples":{"id":"examples","title":"Examples","description":"Usage examples relying on the starknet-devnet-js library can be found here.","sidebar":"docSidebar"},"forking":{"id":"forking","title":"Forking","description":"To interact with contracts deployed on mainnet or testnet, you can use forking. Simulate the origin and experiment with it locally, making no changes to the origin itself.","sidebar":"docSidebar"},"gas":{"id":"gas","title":"Gas price modification","description":"The devnetsetGasPrice RPC method allows users to modify the current gas prices on a running Devnet. This feature is particularly useful for testing purposes and for adjustments needed after forking to align with the forked network\'s gas prices. All parameters are optional, allowing you to choose which ones you want to set. A boolean flag generateblock indicates whether a new block should be generated immediately after setting the gas prices.","sidebar":"docSidebar"},"historic-state":{"id":"historic-state","title":"Historic state support","description":"With state archive capacity set to full, Devnet will store full state history, enabling its querying by block hash or number. The default mode is none, where no old states are stored and only the latest is available for querying.","sidebar":"docSidebar"},"intro":{"id":"intro","title":"Intro","description":"- Devnet should not be used as a replacement for official testnets. After testing on Devnet, be sure to test on a testnet (alpha-sepolia)!","sidebar":"docSidebar"},"lite":{"id":"lite","title":"Lite mode","description":"To run Devnet in a minimal lite mode, provide the flag:","sidebar":"docSidebar"},"postman":{"id":"postman","title":"L1-L2 interaction via Postman","description":"Postman is a Starknet utility that allows testing L1-L2 interaction. It is unrelated to the Postman API platform. Ensure you have an L1 node and a Devnet (L2 node) running, load a messaging contract, and flush the queue to transmit the messages to their destinations. The functionality relies on two internal message queues: one for L1->L2 messages, another for L2->L1 messages.","sidebar":"docSidebar"},"predeployed":{"id":"predeployed","title":"Predeployed contracts","description":"Devnet predeploys a UDC, an ERC20 (fee token) contract and a set of predeployed funded accounts.","sidebar":"docSidebar"},"restrictive":{"id":"restrictive","title":"Restrictive mode","description":"The --restrictive-mode argument enables a restrictive mode for Devnet, allowing you to specify methods that are forbidden during execution. This option ensures that certain operations are restricted, enhancing control over Devnet\'s behavior. When a user sends a request to one of the restricted methods, Devnet will return either a JSON-RPC error with code -32604 or, if the method was targeted directly via the HTTP endpoint, a response with status 403.","sidebar":"docSidebar"},"running/cli":{"id":"running/cli","title":"CLI options","description":"Configure your Devnet instance by specifying CLI parameters on startup. To read more about HTTP and logging configuration, check out the server config page.","sidebar":"docSidebar"},"running/docker":{"id":"running/docker","title":"Run with Docker","description":"Devnet is available as a Docker image (Docker Hub link). To download the latest image, run:","sidebar":"docSidebar"},"running/install":{"id":"running/install","title":"Install and run","description":"Requirements","sidebar":"docSidebar"},"server-config":{"id":"server-config","title":"Server config","description":"To read generally about ways to configure your Devnet instance, check out the CLI section.","sidebar":"docSidebar"},"starknet-time":{"id":"starknet-time","title":"Starknet time","description":"Block and state timestamp can be manipulated by setting the exact time or setting the time offset. By default, timestamp methods /settime, /increasetime and JSON-RPC methods devnetsetTime, devnetincreaseTime generate a new block. This can be changed for /settime (devnetsetTime) by setting the optional parameter generate_block to false. This skips immediate new block generation, but will use the specified timestamp whenever the next block is supposed to be generated.","sidebar":"docSidebar"}}}}')}}]);
"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[2282],{7164:e=>{e.exports=JSON.parse('{"version":{"pluginId":"default","version":"current","label":"Next","banner":"unreleased","badge":true,"noIndex":false,"className":"docs-version-current","isLast":false,"docsSidebars":{"docSidebar":[{"type":"link","label":"Intro","href":"/starknet-devnet-rs/docs/next/intro","docId":"intro","unlisted":false},{"type":"category","label":"Running","collapsible":true,"collapsed":true,"items":[{"type":"link","label":"Install and run","href":"/starknet-devnet-rs/docs/next/running/install","docId":"running/install","unlisted":false},{"type":"link","label":"Run with Docker","href":"/starknet-devnet-rs/docs/next/running/docker","docId":"running/docker","unlisted":false},{"type":"link","label":"CLI options","href":"/starknet-devnet-rs/docs/next/running/cli","docId":"running/cli","unlisted":false}],"href":"/starknet-devnet-rs/docs/next/category/running"},{"type":"link","label":"Account impersonation","href":"/starknet-devnet-rs/docs/next/account-impersonation","docId":"account-impersonation","unlisted":false},{"type":"link","label":"API","href":"/starknet-devnet-rs/docs/next/api","docId":"api","unlisted":false},{"type":"link","label":"Account balance","href":"/starknet-devnet-rs/docs/next/balance","docId":"balance","unlisted":false},{"type":"link","label":"Blocks","href":"/starknet-devnet-rs/docs/next/blocks","docId":"blocks","unlisted":false},{"type":"link","label":"Dump, load, restart","href":"/starknet-devnet-rs/docs/next/dump-load-restart","docId":"dump-load-restart","unlisted":false},{"type":"link","label":"Forking","href":"/starknet-devnet-rs/docs/next/forking","docId":"forking","unlisted":false},{"type":"link","label":"Historic state support","href":"/starknet-devnet-rs/docs/next/historic-state","docId":"historic-state","unlisted":false},{"type":"link","label":"Lite mode","href":"/starknet-devnet-rs/docs/next/lite","docId":"lite","unlisted":false},{"type":"link","label":"L1-L2 interaction via Postman","href":"/starknet-devnet-rs/docs/next/postman","docId":"postman","unlisted":false},{"type":"link","label":"Predeployed contracts","href":"/starknet-devnet-rs/docs/next/predeployed","docId":"predeployed","unlisted":false},{"type":"link","label":"Server config","href":"/starknet-devnet-rs/docs/next/server-config","docId":"server-config","unlisted":false},{"type":"link","label":"Starknet time","href":"/starknet-devnet-rs/docs/next/starknet-time","docId":"starknet-time","unlisted":false}]},"docs":{"account-impersonation":{"id":"account-impersonation","title":"Account impersonation","description":"This page is about account impersonation. To read about account class selection and deployment, click here.","sidebar":"docSidebar"},"api":{"id":"api","title":"API","description":"JSON-RPC API","sidebar":"docSidebar"},"balance":{"id":"balance","title":"Account balance","description":"Other than using prefunded predeployed accounts, you can also add funds to an account that you deployed yourself.","sidebar":"docSidebar"},"blocks":{"id":"blocks","title":"Blocks","description":"Genesis block","sidebar":"docSidebar"},"dump-load-restart":{"id":"dump-load-restart","title":"Dump, load, restart","description":"Dumping","sidebar":"docSidebar"},"forking":{"id":"forking","title":"Forking","description":"To interact with contracts deployed on mainnet or testnet, you can use forking. Simulate the origin and experiment with it locally, making no changes to the origin itself.","sidebar":"docSidebar"},"historic-state":{"id":"historic-state","title":"Historic state support","description":"With state archive capacity set to full, Devnet will store full state history, enabling its querying by block hash or number. The default mode is none, where no old states are stored and only the latest is available for querying.","sidebar":"docSidebar"},"intro":{"id":"intro","title":"Intro","description":"- Devnet should not be used as a replacement for official testnets. After testing on Devnet, be sure to test on a testnet (alpha-sepolia)!","sidebar":"docSidebar"},"lite":{"id":"lite","title":"Lite mode","description":"To run Devnet in a minimal lite mode, provide the flag:","sidebar":"docSidebar"},"postman":{"id":"postman","title":"L1-L2 interaction via Postman","description":"Postman is a Starknet utility that allows testing L1-L2 interaction. Ensure you have an L1 node and a Devnet (L2 node) running, load a messaging contract, and flush the queue when needed. You can use starknet-devnet-js to perform these actions, as witnessed in this example, or directly send requests to the endpoints specified below.","sidebar":"docSidebar"},"predeployed":{"id":"predeployed","title":"Predeployed contracts","description":"Devnet predeploys a UDC, an ERC20 (fee token) contract and a set of predeployed funded accounts.","sidebar":"docSidebar"},"running/cli":{"id":"running/cli","title":"CLI options","description":"Configure your Devnet instance by specifying CLI parameters on startup.","sidebar":"docSidebar"},"running/docker":{"id":"running/docker","title":"Run with Docker","description":"Devnet is available as a Docker image (Docker Hub link). To download the latest image, run:","sidebar":"docSidebar"},"running/install":{"id":"running/install","title":"Install and run","description":"Requirements","sidebar":"docSidebar"},"server-config":{"id":"server-config","title":"Server config","description":"To read generally about ways to configure your Devnet instance, check out the CLI section.","sidebar":"docSidebar"},"starknet-time":{"id":"starknet-time","title":"Starknet time","description":"Block and state timestamp can be manipulated by setting the exact time or setting the time offset. By default, timestamp methods /settime, /increasetime and JSON-RPC methods devnetsetTime, devnetincreaseTime generate a new block. This can be changed for /settime (devnetsetTime) by setting the optional parameter generate_block to false. This skips immediate new block generation, but will use the specified timestamp whenever the next block is supposed to be generated.","sidebar":"docSidebar"}}}}')}}]);
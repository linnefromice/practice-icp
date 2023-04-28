import { HttpAgent } from '@dfinity/agent';
import * as Measurer from "./declarations/measurer";

async function main() {
  const fetch = require('node-fetch')

  const agent = new HttpAgent({
    host: 'http://127.0.0.1:4943',
    fetch
  });
  if(process.env.NODE_ENV !== "production") await agent.fetchRootKey();

  const measurer = Measurer.createActor("rrkah-fqaaa-aaaaa-aaaaq-cai", { agent });
  console.log(await measurer.get_ethereum_address())
}

main();

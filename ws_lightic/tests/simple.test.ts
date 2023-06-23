import { assert } from 'chai'
import { TestContext } from 'lightic'
import { Principal } from '@dfinity/principal'

const context = new TestContext();

describe("Simple test", () => {
  afterEach(function () {
    context.clean() // clean lightic
  })

  it("Deploy", async () => {
    await context.deploy('./modules/hello_motoko/hello_motoko.wasm')
    await context.deploy('./modules/hello_rust/hello_rust.wasm')
  
    const canisters = context.replica.get_canisters()
    // there is management canister installed by default
    assert.equal(canisters.length, 3)
  })

  it("Call function", async () => {
    const caller = Principal.anonymous()
    const canister = await context.deploy('./modules/counter_motoko/counter_motoko.wasm');
    const actor = context.getAgent(caller).getActor(canister)

    const res = await actor.get() as any[]
    console.log(res)
  })
})
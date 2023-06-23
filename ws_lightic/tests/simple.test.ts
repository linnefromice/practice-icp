import { assert } from 'chai'
import { TestContext } from 'lightic'

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
})
import { assert } from 'chai'

describe("Example test", () => {
  it("should pass", () => {
    assert.equal(2 + 3, 5)
  })
  it("should fail", () => {
    assert.notEqual(2 + 3, 6)
  })
})
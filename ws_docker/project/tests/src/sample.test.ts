import {expect, test} from 'vitest';
import {
  helloMotoko,
  helloRust,
  counterMotoko,
  counterRust,
  httpOutcallErc20,
  httpOutcallPool
} from './actor.js';

test('sample', () => {
  const sum = 1 + 1;
  expect(sum).toBe(2);
});

test('sample with canister', async () => {
  const msgFromHelloMotoko = await helloMotoko.greet('Motoko');
  expect(msgFromHelloMotoko).toBe('Hello, Motoko! from Motoko');
  const msgFromHelloRust = await helloRust.greet('Rust');
  expect(msgFromHelloRust).toBe('Hello, Rust! from Rust');
  const msgFromCounterMotoko = await counterMotoko.name();
  expect(msgFromCounterMotoko).toBe('CounterMotoko');
  const msgFromCounterRust = await counterRust.name();
  expect(msgFromCounterRust).toBe('CounterRust');
});

// 9546ms
test('erc20', async () => {
  expect(await httpOutcallErc20.name()).toBe("Dai Stablecoin");
  expect(await httpOutcallErc20.symbol()).toBe("DAI");
});

// 19076ms
test('pool', async () => {
  expect(await httpOutcallPool.token0()).toBe("a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48");
  expect(await httpOutcallPool.token1()).toBe("c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2");
  expect(await httpOutcallPool.fee()).toBe(500);
  const slot0 = await httpOutcallPool.slot0();
  console.log(slot0);
});

// 8298ms
test('counter by Motoko', async () => {
  const counter = counterMotoko;
  await counter.reset();
  expect(await counter.get()).toBe(BigInt(0));
  await counter.inc();
  await counter.inc();
  await counter.inc();
  expect(await counter.get()).toBe(BigInt(3));
});

test('counter by Rust', async () => {
  const counter = counterRust;
  await counter.reset();
  expect(await counter.get()).toBe(BigInt(0));
  await counter.inc();
  await counter.inc();
  await counter.inc();
  expect(await counter.get()).toBe(BigInt(3));
});

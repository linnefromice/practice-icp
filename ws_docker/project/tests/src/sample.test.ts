import {expect, test} from 'vitest';
import {
  helloMotoko,
  helloRust,
  counterMotoko,
  counterRust,
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

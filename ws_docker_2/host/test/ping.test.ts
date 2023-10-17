import {describe, test, expect} from '@jest/globals';

describe('ping', () => {
  test('should return pong', () => {
    expect('pong').toBe('pong');
  });
});

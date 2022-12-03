import { input } from '../input.ts';

const Shapes: Record<string, number> = {
  A: 1,
  X: 1,
  B: 2,
  Y: 2,
  C: 3,
  Z: 3,
};

const split = input.split('\n').map((v) => v.split(' '));

const partOne = () => {
  return split.map(([leftShape, rightShape]) => {
    const left = Shapes[leftShape];
    const right = Shapes[rightShape];
    const diff = Math.abs(left - right);

    if (left === right) {
      return right + 3;
    }

    if ((diff === 1 && right > left) || (right === 1 && left === 3)) {
      return right + 6;
    }

    return right;
  }).reduce((a, b) => a + b);
};

const partTwo = () => {
  return split.map(([leftShape, rightShape]) => {
    const left = Shapes[leftShape];

    if (rightShape === 'X') {
      return left - 1 || 3;
    }

    if (rightShape === 'Y') {
      return left + 3;
    }

    return ((left + 1) % 3 || 3) + 6;
  }).reduce((a, b) => a + b);
};

console.log(`Part 1: ${partOne()}`);
console.log(`Part 2: ${partTwo()}`);

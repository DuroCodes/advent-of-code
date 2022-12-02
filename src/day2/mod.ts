const Shapes: Record<string, number> = {
  A: 1,
  X: 1,
  B: 2,
  Y: 2,
  C: 3,
  Z: 3,
};

const input = (await Deno.readTextFile('input.txt')).trim();
const split = input.split('\n').map((v) => v.split(' '));

const partOneRounds = split.map(([leftShape, rightShape]) => {
  const left = Shapes[leftShape];
  const right = Shapes[rightShape];
  const diff = Math.abs(left - right);

  // Tie
  if (left === right) {
    return right + 3;
  }

  // Win
  if ((diff === 1 && right > left) || (right === 1 && left === 3)) {
    return right + 6;
  }

  // Lose
  return right;
});

const partTwoRounds = split.map(([leftShape, rightShape]) => {
  const left = Shapes[leftShape];

  // Lose
  if (rightShape === 'X') {
    return left - 1 || 3;
  }

  // Tie
  if (rightShape === 'Y') {
    return left + 3;
  }

  // Win
  return ((left + 1) % 3 || 3) + 6;
});

console.log(`Part 1: ${partOneRounds.reduce((a, b) => a + b)}`);
console.log(`Part 2: ${partTwoRounds.reduce((a, b) => a + b)}`);

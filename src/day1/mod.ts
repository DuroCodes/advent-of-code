const input = (await Deno.readTextFile('input.txt')).trim();

const calories = input
  .split('\n\n')
  .map((x) => x.split('\n'))
  .map((arr) => arr.map((x) => parseInt(x))).map((v) =>
    v.reduce((a, b) => a + b)
  );

const sorted = calories.sort((a, b) => b - a).slice(0, 3);

const partOne = () => (sorted[0]);
const partTwo = () => (sorted.reduce((a, b) => a + b));

console.log(`Part 1: ${partOne()}`);
console.log(`Part 2: ${partTwo()}`);

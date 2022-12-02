const input = (await Deno.readTextFile('input.txt')).trim();

const calories = input
  .split('\n\n')
  .map((x) => x.split('\n'))
  .map((arr) => arr.map((x) => parseInt(x))).map((v) =>
    v.reduce((a, b) => a + b)
  );

const sorted = calories.sort((a, b) => b - a).slice(0, 3);

console.log(`Part 1: ${sorted[0]}`);
console.log(`Part 2: ${sorted.reduce((a, b) => a + b)}`);

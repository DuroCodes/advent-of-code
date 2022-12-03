import { input } from '../input.ts';

const split = input.split('\n');

const letterPriorities = (letter: string) => {
  return /[a-z]/.test(letter)
    ? letter.charCodeAt(0) - 96
    : letter.charCodeAt(0) - 65 + 27;
};

const partOne = () => {
  const res = split.map((line) => {
    const [first, second] = [
      [...line.slice(0, line.length / 2)],
      [...line.slice(line.length / 2)],
    ];

    const firstSet = new Set(first);

    return letterPriorities(
      [...new Set(second.filter((v) => firstSet.has(v)))][0],
    );
  });

  return res.reduce((a, b) => a + b);
};

const partTwo = () => {
  let sum = 0;

  for (let i = 0; i < split.length; i += 3) {
    const backpacks = [[...split[i]], [...split[i + 1]], [...split[i + 2]]];

    let set = new Set(backpacks[0]);
    let intersection = backpacks[1].filter((v) => set.has(v));

    set = new Set(intersection);
    intersection = backpacks[2].filter((v) => set.has(v));

    const dedup = [...new Set(intersection)];

    sum += letterPriorities(dedup[0]);
  }

  return sum;
};

console.log(`Part 1: ${partOne()}`);
console.log(`Part 2: ${partTwo()}`);

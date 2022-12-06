import { input } from '../input.ts';

const split = input.split('');

const findIndex = (packetSize = 4) => {
  const markerIndex = split.findIndex((_, i) => {
    const window = input.slice(i, i + packetSize);
    return [...new Set(window)].length === window.length;
  });

  return markerIndex + packetSize;
};

const partOne = () => findIndex();
const partTwo = () => findIndex(14);

console.log(`Part 1: ${partOne()}`);
console.log(`Part 1: ${partTwo()}`);

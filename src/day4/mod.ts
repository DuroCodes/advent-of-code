import { input } from '../input.ts';

interface Line {
  start: number;
  end: number;
}

const split = input.split('\n').map((v) => v.split(','));

const ranges: Line[][] = split.map((e) =>
  e.map((v) => {
    const vals = v.split('-');

    return {
      start: parseInt(vals[0]),
      end: parseInt(vals[1]),
    };
  })
);

const partOne = () => {
  const checkOverlap = (first: Line, second: Line) => (
    first.start >= second.start && first.end <= second.end ||
    second.start >= first.start && second.end <= first.end
  );

  return ranges.map(
    ([first, second]) => checkOverlap(first, second),
  ).filter(Boolean).length;
};

const partTwo = () => {
  const checkOverlap = (first: Line, second: Line) => (
    first.start >= second.start && first.start <= second.end ||
    first.end >= second.start && first.end <= second.end ||
    second.start >= first.start && second.start <= first.end ||
    second.end >= first.start && second.end <= first.end
  );

  return ranges.map(
    ([first, second]) => checkOverlap(first, second),
  ).filter(Boolean).length;
};

console.log(`Part 1: ${partOne()}`);
console.log(`Part 2: ${partTwo()}`);

import { input } from '../input.ts';

const split = input.split('\n');

const getStacks = () => {
  const stacks: string[][] = [];
  const numStacks = Math.ceil(split[0].length / 4);

  for (let i = 0; i < numStacks; i++) {
    stacks.push([]);
  }

  const containers: string[] = [];
  const instructions: string[] = [];
  let status = 'containers';

  for (let i = 0; i < split.length; i++) {
    if (split[i][1] === '1') {
      status = 'split';
    }

    if (split[i][0] === 'm') {
      status = 'instructions';
    }

    if (status === 'containers') {
      containers.push(split[i]);
    } else if (status === 'instructions') {
      instructions.push(split[i]);
    }
  }

  containers.forEach((row) => {
    const parts = row.split(' ');
    let pos = 0;

    parts.forEach((part) => {
      if (part === '') return pos++;
      if (part.substring(0, 1) === '[') pos += 4;

      const num = pos / 4 - 1;
      stacks[num].unshift(part);
    });
  });

  return { stacks, instructions };
};

const getInstruction = (row: string) => {
  const instruction = row.split(' ');
  const amount = parseInt(instruction[1]);
  const fromStack = parseInt(instruction[3]) - 1;
  const toStack = parseInt(instruction[5]) - 1;

  return { amount, fromStack, toStack };
};

const runInstructions = (stacks: string[][], instructions: string[]) => {
  instructions.forEach((row) => {
    const { amount, fromStack, toStack } = getInstruction(row);

    for (let i = 0; i < amount; i++) {
      stacks[toStack].push(stacks[fromStack].pop()!);
    }
  });
};

const runTrueInstructions = (stacks: string[][], instructions: string[]) => {
  instructions.forEach((row) => {
    const { amount, fromStack, toStack } = getInstruction(row);
    const remove = stacks[fromStack].splice(
      stacks[fromStack].length - amount,
      amount,
    );
    stacks[toStack] = stacks[toStack].concat(remove);
  });
};

const topOfStack = (stacks: string[][]) => {
  let res = '';
  stacks.forEach((stack) => {
    res += stack[stack.length - 1].substring(1, 2);
  });

  return res;
};

const partOne = () => {
  const { stacks, instructions } = getStacks();
  runInstructions(stacks, instructions);

  return topOfStack(stacks);
};

const partTwo = () => {
  const { stacks, instructions } = getStacks();
  runTrueInstructions(stacks, instructions);

  return topOfStack(stacks);
};

console.log(`Part 1: ${partOne()}`);
console.log(`Part 2: ${partTwo()}`);

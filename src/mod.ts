import {
  Select,
  SelectOption,
} from 'https://deno.land/x/cliffy@v0.25.4/prompt/select.ts';

const days: SelectOption[] = [];

for await (const { name: value, isDirectory } of Deno.readDir('./src')) {
  if (isDirectory && value.startsWith('day')) {
    days.push({ name: value.replace('day', 'Day '), value });
  }
}

const options = days.reverse().sort(({ name: first }, { name: second }) => {
  const [_, firstNum] = first?.split(' ').map((v) => parseInt(v))!;
  const [__, secondNum] = second?.split(' ').map((v) => parseInt(v))!;

  return firstNum < secondNum ? -1 : 1;
});

const day = await Select.prompt({
  message: 'Which day would you like to run?',
  search: true,
  options,
});

await Deno.run({
  cmd: ['deno', 'run', '--allow-read', `./src/${day}/mod.ts`],
  cwd: './',
}).status();

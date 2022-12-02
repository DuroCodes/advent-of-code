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

const day = await Select.prompt({
  message: 'Which day would you like to run?',
  options: days.reverse(),
  search: true,
});

await Deno.run({
  cmd: ['deno', 'run', '--allow-read', `./src/${day}/mod.ts`],
  cwd: './',
}).status();

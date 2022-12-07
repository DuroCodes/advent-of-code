import { input } from '../input.ts';

interface File {
  size: number;
  name: string;
}

interface Folder {
  name: string;
  children: Folder[];
  files: File[];
  parent?: Folder;
}

const toFolder = ({ lineInput, workingDir, dirTree }: {
  lineInput: string[];
  workingDir: Folder;
  dirTree: Folder;
}) => {
  if (lineInput[0] !== 'cd') return workingDir;

  const arg = lineInput[1];
  if (arg === '/') return dirTree;
  if (arg === '..') return workingDir.parent!;
  if (arg.match(/[a-z]/)) return workingDir.children.find(({ name }) => name === arg)!;

  return workingDir;
};

const parseDir = (lineInput: string[], workingDir: Folder) => (
  workingDir.children.push({
    name: lineInput[0],
    children: [],
    files: [],
    parent: workingDir,
  })
);

const parseFile = (lineInput: string[], workingDir: Folder) => (
  workingDir.files.push({ size: parseInt(lineInput[0]), name: lineInput[1] })
);

const folderSize = (folder: Folder): { name: string; size: number; }[] => {
  const size = folder.files.reduce((a, b) => a += b.size, 0);
  const childDir = folder.children.map((v) => folderSize(v));
  const childDirSize = childDir.map((v) => v[0]).reduce((a, b) => a += b.size, 0);

  return [
    { name: folder.name, size: size + childDirSize },
    ...childDir.flat(),
  ];
};

const fileInput = input.split('\n');

const dirTree: Folder = { name: '/', children: [], files: [] };
let workingDir = dirTree;

fileInput.forEach((cmd) => {
  const lineInput = cmd.split(' ');

  if (lineInput[0] === '$') {
    return workingDir = toFolder({ lineInput: lineInput.slice(1), workingDir, dirTree });
  }

  if (lineInput[0] === 'dir') {
    parseDir(lineInput.slice(1), workingDir);
  }

  if (lineInput[0].match(/[0-9]/)) {
    parseFile(lineInput, workingDir);
  }
});

const folders = folderSize(dirTree);

const partOne = () => (
  folders.filter(
    ({ size }) => size <= 100_000,
  ).reduce((a, b) => a + b.size, 0)
);

const partTwo = () => {
  const sizeToDelete = folders.find(({ name }) => name === '/')!.size - 40_000_000;
  return folders
    .filter(({ size }) => size - sizeToDelete > 0)
    .sort((a, b) =>
      sizeToDelete - a.size < sizeToDelete - b.size ? 1 : -1
    )[0].size;
};

console.log(`Part 1: ${partOne()}`);
console.log(`Part 2: ${partTwo()}`);

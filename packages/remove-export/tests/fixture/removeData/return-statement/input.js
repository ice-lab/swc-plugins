import fs from 'fs'

const [a] = fs.promises

const bar = '123';

export async function getData() {
  return bar;
}

export function getConfig() {
  console.log(1)
}

export default class Home {
  constructor() {
    console.log(a);
  }
}

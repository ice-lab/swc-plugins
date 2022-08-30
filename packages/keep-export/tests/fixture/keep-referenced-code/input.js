import fs from 'fs'

const [a, b, ...rest] = fs.promises

export async function getData() {
  a
  b
}

function getConfig() {
  console.log(rest)
}

export const name1 = getConfig;

export default class Home {
  constructor() {
    console.log(a);
  }
}

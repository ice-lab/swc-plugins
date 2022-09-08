import fs from 'fs'

const [a, b, ...rest] = fs.promises

export async function getData() {
  a
  b
  rest
}

function getConfig() {
  console.log(1)
}

export const name1 = getConfig;

export default class Home {
  constructor() {
    console.log(a);
  }
}

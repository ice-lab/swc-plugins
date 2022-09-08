import fs from 'fs'

const [a, b, c, ...rest] = fs.promises

const data = {
  id: 123
}

const { name } = c;

export async function getData() {
  return {
    a,
    b,
    id: data.id,
    name
  };
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

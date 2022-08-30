import fs from 'fs'

const [a, b, ...rest] = fs.promises

export async function getData() {
  console.log(1)
}

export function getConfig() {
  a
  b
  rest
}

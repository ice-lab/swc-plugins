import fs from 'fs'
import other from 'other'

const [a, b, ...rest] = fs.promises
const [foo, bar] = other

export async function getData() {
  a
  b
  rest
  bar
}

export function getConfig() {
}

export default () => {
  console.log('hello');
}

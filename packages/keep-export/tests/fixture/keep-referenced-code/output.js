import fs from 'fs'
const [a, b, c] = fs.promises
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
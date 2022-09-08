import { isNode } from "env";

let a;

try {
  a = 123;
} catch {
  throw new Error("oops");
}

export const getData = () => {
  return "123";
}

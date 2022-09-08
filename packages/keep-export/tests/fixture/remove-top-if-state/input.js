import { isNode } from "env";

let url;

if (isNode) {
  url = "aaa";
}

export const getData = () => {
  return "123";
}


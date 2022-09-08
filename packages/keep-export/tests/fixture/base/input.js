const a = 123;

const data = {};
data.id = 123;

export const getData = () => {
  return "123";
}

export const getConfig = () => {
  return {
    title: ""
  }
}

export default class Home {
  constructor() {
    console.log(a);
  }
}

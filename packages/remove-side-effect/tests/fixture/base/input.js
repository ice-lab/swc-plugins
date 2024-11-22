import { useEffect, useLayoutEffect } from "react";

const Component = () => {
  useEffect(() => {
    console.log("Hello");
  }, []);

  useLayoutEffect(() => {
    console.log("Hello Layout");
  }, []);

  return <div>Hello</div>
}

function Component2() {
  useEffect(() => {
    console.log("Hello");
  }, []);

  useLayoutEffect(() => {
    console.log("Hello Layout");
  }, []);

  return <div>Hello</div>
}

export { Component2 };

export default Component;
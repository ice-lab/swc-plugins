import { useEffect, useLayoutEffect } from "react";

const Component = () => {
  // can't find useEffect, because the Temporal Dead Zone, TDZ, but we should not to remove it
  // because it's a local variable, not form react
  useEffect(() => {
    console.log("Hello");
  }, []);

  useLayoutEffect(() => {
    console.log("Hello Layout");
  }, []);

  const useEffect = () => {}
  useEffect()

  return <div>Hello</div>
}

export default Component;
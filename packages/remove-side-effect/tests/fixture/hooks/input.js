import { useEffect, useLayoutEffect } from "react";

function useCustomHook() {
  useEffect(() => {
    console.log("Custom Hook Effect");
  }, []);

  useLayoutEffect(() => {
    console.log("Hello Layout");
  }, []);

  {
    const useEffect = () => {}
    useEffect()
  }
}

const Component = () => {
  useCustomHook();
  return <div>Hello</div>
}

export default Component;
import { useEffect, useLayoutEffect } from "react";

function useCustomHook() {

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
import { useEffect } from "react";

const Component = () => {

  {
    const useEffect = () => {}
    useEffect()
  }

  try {
    const useEffect = () => {}
    useEffect()
  } catch (e) {}

  const B = () => {
    const useEffect = () => {
      console.log('another UseEffect');
    };
    useEffect();
  }

  B();

  return <div>Hello</div>
}

export default Component;
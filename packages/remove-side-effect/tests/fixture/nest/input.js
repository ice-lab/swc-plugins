import { useEffect } from "react";

const Component = () => {
  useEffect(() => {
    console.log("Hello");

    const useEffect = () => {}
    useEffect()
  }, []);

  {
    const useEffect = () => {}
    useEffect()
  }

  try {
    const useEffect = () => {}
    useEffect()
  } catch (e) {
    // React useEffect
    useEffect(() => {})
  }

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
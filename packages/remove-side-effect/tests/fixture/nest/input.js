import { useEffect } from "react";

const Component = () => {
  useEffect(() => {
    console.log("Hello");
  }, []);

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
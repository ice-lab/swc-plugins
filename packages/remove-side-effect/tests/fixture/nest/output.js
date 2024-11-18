import { useEffect } from "react";

const Component = () => {

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
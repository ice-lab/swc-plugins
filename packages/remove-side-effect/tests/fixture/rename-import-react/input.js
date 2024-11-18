import { useEffect as myEffect, useLayoutEffect as myLayout } from "react";
import { 'useEffect' as myEffect2 } from 'react';


const Component = () => {
  myEffect2(() => {
    console.log("Hello");
  }, []);

  myEffect(() => {
    console.log("Hello");
  }, []);

  myLayout(() => {
    console.log("Layout");
  }, []);

  return <div>Hello</div>
}

export default Component;
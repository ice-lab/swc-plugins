import { useEffect, useCallback } from "react";

const Component = () => {
  useEffect(() => {
    console.log("Hello");
  }, []);

  return <div>Hello</div>
}

export default Component;
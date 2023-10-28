import React, { useState } from "react";
export default function App() {
  const [response, setResponse] = useState("");

  const onClick = async (
    e: React.MouseEvent<HTMLButtonElement, MouseEvent>,
  ) => {
    const res = await fetch("https://mybinary.localhost/");
    const decoded = res.body!.pipeThrough(new TextDecoderStream("utf8"));

    setResponse((await decoded.getReader().read()).value!);
  };

  return (
    <div className="container">
      <button onClick={onClick}>
        send custom protocol request!
      </button>
      <p>{response}</p>
    </div>
  );
}

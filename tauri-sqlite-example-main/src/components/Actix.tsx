import React, { useEffect, useState } from "react";
import axios from "axios";

function Actix() {
  const [message, setMessage] = useState("");

  useEffect(() => {
    axios.get("http://127.0.0.1:7878/api/hello")
      .then(res => setMessage(res.data))
      .catch(err => console.error("Error:", err));
  }, []);

  return (
    <div>
      
      <p><h1>{message}</h1></p>
    </div>
  );
}

export default Actix;
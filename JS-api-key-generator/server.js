const express = require("express");
const app = express();

const BASE = "/api/generator/";
const PORT = 8000;

const CHARSET =
  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~.";

app.get(BASE + "key", (req, res) => {
  const LENGTH = parseInt(req.query.length);
  let result = "";

  for (let i = 0; i < LENGTH; i++) {
    result += CHARSET.charAt(Math.floor(Math.random() * CHARSET.length));
  }
  res.send({ key: result });
  console.log(`Request handled!`)
});

app.listen(PORT, () => {
  console.log(`Running a server on http://localhost:${PORT}`);
});

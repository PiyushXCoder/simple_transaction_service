const http = require("http");

const PORT = process.env.PORT || 3000;

const server = http.createServer((req, res) => {
  let body = "";
  req.on("data", (chunk) => {
    body += chunk.toString();
  });
  req.on("end", () => {
    console.log("Received webhook:", body);
    res.writeHead(200, { "Content-Type": "text/plain" });
    res.end("Webhook received\n");
  });
});

server.listen(PORT, () => {
  console.log(`Webhook server is listening on port ${PORT}`);
});

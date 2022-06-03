const WebSocket = require('ws');
const client = new WebSocket("ws://127.0.0.1:9998/Service/stateful_async", ["jsonrpc"]);

next_id = 10
pending = {}

function send_message(method, params) {
  return new Promise((resolve, reject) => {
    let req = {
      jsonrpc: "2.0",
      id: next_id++,
      method,
      params
    }
    const s = JSON.stringify(req);
    console.log("send: " + s)
    pending[req.id] = resolve
    client.send(s)
  })
}

client.onopen = async function (e) {
  await send_message("State.get")
  await send_message("State.set", "NewState")
  await send_message("State.get")
  await send_message("State.set", "NewState2")
  await send_message("State.get")
  client.close()
}

client.onmessage = function (msg) {
  console.log("recv:" + msg.data + "\n")
  const resp = JSON.parse(msg.data)
  let pend = pending[resp.id]
  if (pend) {
    pend(resp.result)
  }
}

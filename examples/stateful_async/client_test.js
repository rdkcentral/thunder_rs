/*
 * Copyright 2022 Comcast Cable Communications Management, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */
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

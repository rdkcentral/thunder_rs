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
const client = new WebSocket("ws://127.0.0.1:55555/Service/hello_world", ["jsonrpc"]);

next_id = 10

function send_message() {
  let req = {};
  req.jsonrpc = "2.0"
  req.id = next_id++

  if (next_id % 2 == 0) {
    req.method = "settings.onRequestSettings",
    req.params = [1, 2, 3, 4, 5];
  }
  else {
    req.method = "keyboard.onKeyPress",
    req.params = { 
      a: "a",
      b: "b"
    }
  }

  const s = JSON.stringify(req);
  console.log("send:" + s)
  client.send(s)
  setTimeout(send_message, 1000)
}

client.onopen = function(e) {
  send_message()
}

client.onmessage = function(msg) {
  console.log("recv:" + msg.data)
  const req = JSON.parse(msg.data)
  // console.log("recv:" + JSON.stringify(req))
}

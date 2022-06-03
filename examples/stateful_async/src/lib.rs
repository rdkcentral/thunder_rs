use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{runtime::Runtime, sync::mpsc};

/*
 * If not stated otherwise in this file or this component's LICENSE file the
 * following copyright and licenses apply:
 *
 * Copyright 2022 RDK Management
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
 */
struct StatefulPlugin {
    plugin_tx: mpsc::Sender<PluginRequest>,
    _rt: Runtime,
}

struct PluginRequest {
    json: String,
    ctx: thunder_rs::RequestContext,
}

struct StatefulService {
    state: String,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: u32,
    result: Value,
}

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: u32,
    method: String,
    params: Option<Value>,
}

impl StatefulService {
    async fn state_async(&self) -> String {
        String::from(self.state.clone())
    }
    fn set_state(&mut self, state: String) {
        self.state = state
    }

    async fn handle_rpc(&mut self, jsonrpc: String) -> String {
        let req: JsonRpcRequest = serde_json::from_str(&jsonrpc).unwrap();
        println!("Handle RPC call {}", req.method.as_str());
        let val = match req.method.as_str() {
            "State.set" => {
                let ps = req.params.unwrap();
                let new_state = ps.as_str().unwrap();
                self.set_state(String::from(new_state));
                Value::Null
            }
            "State.get" => Value::String(self.state_async().await),
            _ => {
                //TODO Should really return an error
                Value::Null
            }
        };
        let resp = JsonRpcResponse {
            jsonrpc: String::from("2.0"),
            id: req.id,
            result: val,
        };
        serde_json::to_string(&resp).unwrap()
    }
}

impl thunder_rs::Plugin for StatefulPlugin {
    fn on_message(&mut self, json: String, ctx: thunder_rs::RequestContext) {
        println!("\tchannel:{0}", ctx.channel);
        println!("\tauth_token:{0}", ctx.auth_token);
        let tx = self.plugin_tx.clone();
        async_std::task::block_on(async {
            let r = tx
                .send(PluginRequest {
                    json: json,
                    ctx: ctx,
                })
                .await;
            match r {
                Ok(_) => println!("Successfully sent!"),
                Err(err) => println!("ERR {}", err),
            }
        });
    }

    fn on_client_connect(&mut self, channel: u32) {
        println!("client_connect:{}", channel);
    }

    fn on_client_disconnect(&mut self, channel: u32) {
        println!("client_disconnect:{}", channel);
    }
}

fn stateful_plugin_init() -> Box<dyn thunder_rs::Plugin> {
    let _rt = Runtime::new().unwrap();
    let _guard = _rt.enter();

    let (plugin_tx, mut plugin_rx) = mpsc::channel::<PluginRequest>(32);
    tokio::spawn(async move {
        let mut service = StatefulService {
            state: String::from("Initial"),
        };
        while let Some(msg) = plugin_rx.recv().await {
            let s = service.handle_rpc(msg.json).await;
            println!("Sending {}", s);
            msg.ctx
                .responder
                .send(thunder_rs::Message {
                    channel: msg.ctx.channel,
                    data: s,
                })
                .unwrap();
        }
    });
    Box::new(StatefulPlugin { plugin_tx, _rt })
}

thunder_rs::export_plugin!("StatefulRustPlugin", (1, 0, 0), stateful_plugin_init);

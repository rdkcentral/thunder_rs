# thunder_rs

A Rust Thunder Plugin Adapter. This plugin allows developes to build Thunder plugins using the Rust programming language. 

# Desktop build and run instructions using Thunder WPEFramework

## build Thunder 

Following steps 1 and 2 from https://github.com/rdkcentral/Thunder.  Repeated below:

```
export THUNDER_ROOT=${HOME}/thunder
export THUNDER_INSTALL_DIR=${THUNDER_ROOT}/install
mkdir -p ${THUNDER_INSTALL_DIR}
cd ${THUNDER_ROOT}

git clone https://github.com/rdkcentral/Thunder.git -b R2

cmake -HThunder/Tools -Bbuild/ThunderTools \
      -DCMAKE_INSTALL_PREFIX=${THUNDER_INSTALL_DIR}/usr \
      -DCMAKE_MODULE_PATH=${THUNDER_INSTALL_DIR}/tools/cmake \
      -DGENERIC_CMAKE_MODULE_PATH=${THUNDER_INSTALL_DIR}/tools/cmake 

make -C build/ThunderTools && make -C build/ThunderTools install

cmake -HThunder -Bbuild/Thunder \
      -DCMAKE_INSTALL_PREFIX=${THUNDER_INSTALL_DIR}/usr \
      -DCMAKE_MODULE_PATH=${THUNDER_INSTALL_DIR}/tools/cmake \
      -DBUILD_TYPE=Debug -DBINDING=127.0.0.1 -DPORT=55555

make -C build/Thunder && make -C build/Thunder install
```

## build rdkservices 

rdkservices has the RustAdapter C++ plugin which allows a Rust plugin to be loaded through Thunder

From inside the THUNDER_ROOT (from the last step):

```
git clone https://github.com/rdkcentral/rdkservices -b sprint/2205

cmake -Hrdkservices -Bbuild/rdkservices \
  -DCMAKE_INSTALL_PREFIX=${THUNDER_INSTALL_DIR}/usr \
  -DCMAKE_MODULE_PATH=${THUNDER_INSTALL_DIR}/tools/cmake \
  -DCOMCAST_CONFIG=OFF \
  -DPLUGIN_RUSTADAPTER=ON

make -C build/rdkservices && make -C build/rdkservices install
```

Confirm that `${THUNDER_INSTALL_DIR}/usr/lib/wpeframework/plugins/libWPEFrameworkRustAdapter.so` exists. This is the acutal
Thunder plugin that bridges (adapts) the native C++ interaces to rust traits, structs, and other types.

Confirm that `${THUNDER_INSTALL_DIR}/usr/bin/rust_adapter_process` exist.  This is the remote process which we be spawned
when running the plugin in outofprocess mode.

## Build and install the example plugin (rust plugin)

The next build step should produce a libarary named `libhello_world.so` in the build tree. This is the actual rust plugin. 
This file and any dependencies that you may have added are required to be in the `LD_LIBRARY_PATH`. 
To run like normal C++ plugins, we suggest that this be placed into the plugins directory under the 
${THUNDER_INSTALL_DIR}/etc/WPEFramework/plugins, but this is not strictly necessary and is inconvenient during edit, compile, test cycles.

Thunder however, requires that the configuration file for the plugin be installed into ${THUNDER_INSTALL_DIR}/etc/WPEFramework/plugins directory.

```
git clone https://github.com/rdkcentral/thunder_rs.git -b sprint/2205
cargo build --manifest-path ${THUNDER_ROOT}/thunder_rs/examples/hello_world/Cargo.toml --target-dir ${THUNDER_ROOT}/build/thunder_rs/examples/hello_world
cp ${THUNDER_ROOT}/build/thunder_rs/examples/hello_world/debug/libhello_world.so ${THUNDER_INSTALL_DIR}/usr/lib/plugins
cp ${THUNDER_ROOT}/thunder_rs/examples/hello_world\SampleRustPlugin.json ${THUNDER_INSTALL_DIR}/etc/WPEFramework/plugins
```

## test with example client

There's a nodejs application in the examples directory that can be used to test out the HelloWorld plugin. 
This app makes a WebSocket connection to Thunder and repeatedly (1/sec) sends JSON/RPC requests to the plugin and gets "Hell from rust" back. 

### Setup the sample client

```
mkdir ${THUNDER_ROOT}/sample_plugin_client
cp ${THUNDER_ROOT}/thunder_rs/examples/hello_world/sample_plugin_client.js ${THUNDER_ROOT}/sample_plugin_client
pushd ${THUNDER_ROOT}/sample_plugin_client
npm install ws
popd
```

### Launch WPEFramework

```
PATH=${THUNDER_INSTALL_DIR}/usr/bin:${PATH} \
LD_LIBRARY_PATH=${THUNDER_INSTALL_DIR}/usr/lib:${THUNDER_INSTALL_DIR}/usr/lib/plugins:${LD_LIBRARY_PATH} \
WPEFramework -c ${THUNDER_INSTALL_DIR}/etc/WPEFramework/config.json
```

Verify the plugin loads by looking for output like this:

```
[1212546] INFO [/data/thunder/Thunder/rdkservices_mark/RustAdapter/RustAdapter.cpp:45] Initialize: RustAdapter::Initialize OutOfProc=false
[1212546] DEBUG [/data/thunder/Thunder/rdkservices_mark/RustAdapter/LocalPlugin.cpp:90] find_rust_plugin: Loading library from:/data/thunder/Thunder/install/usr/lib/plugins/libhello_world.so
[     137550 us] Activated plugin [RustAdapter]:[hello_world]
```

### Launch the sample client

In another console run this:

```
pushd ${THUNDER_ROOT}/sample_plugin_client
node sample_plugin_client.js
popd
```

Verify the client prints out valid results like this:

```
send:{"jsonrpc":"2.0","id":10,"method":"keyboard.onKeyPress","params":{"a":"a","b":"b"}}
recv:{"jsonrpc":"2.0","id":4,"result":"hello from rust"}
send:{"jsonrpc":"2.0","id":11,"method":"settings.onRequestSettings","params":[1,2,3,4,5]}
recv:{"jsonrpc":"2.0","id":4,"result":"hello from rust"}
```

To end the test hit 'q' in the WPEFramework console.

### Test out of process

The previous step ran the Rust plugin in the same process as WPEFramework. To run it out of process the outofprocess, set
the "outofprocess" field to true in ${THUNDER_INSTALL_DIR}/etc/WPEFramework/plugins/SampleRustPlugin.json

To test, repeat steps "Launch WPEFramework" and "Launch the sample client" and verify results.

# Extra Commands:

## Build the rust_adapter_process directly

### To pick up new changes from thunder_rs repo
cargo update --manifest-path ${THUNDER_ROOT}/rdkservices/RustAdapter/rust_adapter_process/Cargo.toml

### Build it
cargo build --manifest-path ${THUNDER_ROOT}/rdkservices/RustAdapter/rust_adapter_process/Cargo.toml \
--target-dir ${THUNDER_ROOT}/build/rdkservices/RustAdapter/rust_adapter_process

cp ${THUNDER_ROOT}/build/rdkservices/RustAdapter/rust_adapter_process/debug/rust_adapter_process ${THUNDER_INSTALL_DIR}/usr/bin


# TODO
- Check this. Is there a way to configure Thunder to search other directories for plugin config files?
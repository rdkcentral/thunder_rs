# thunder_rs

A Rust Thunder Plugin Adapter. This plugin allows developes to build Thunder plugins using the Rust programming language. 

prerequisite build Thunder following steps 1 and 2 from https://github.com/rdkcentral/Thunder

### Building Thunder RustAdapter Service
---

#### **1. Setup Workspace**
```
export THUNDER_ROOT=${HOME}/thunder
export THUNDER_INSTALL_DIR=${THUNDER_ROOT}/install
mkdir -p ${THUNDER_INSTALL_DIR}
cd ${THUNDER_ROOT}
```

#### **2. Build the Adapter libWPEFrameworkRustAdapter (native plugin)**

```
git clone https://github.com/mrcomcast123/thunder_rs.git
cmake -Hthunder_rs/service -Bbuild/thunder_rs 
      -DCMAKE_INSTALL_PREFIX=${THUNDER_INSTALL_DIR}/usr
      -DCMAKE_MODULE_PATH=${THUNDER_INSTALL_DIR}/tools/cmake     
make -C build/thunder_rs && make -C build/thunder_rs install
```

Confirm that `${THUNDER_INSTALL_DIR}/usr/lib/wpeframework/plugins/libWPEFrameworkRustAdapter.so` exists. This is the acutal
Thunder plugin that bridges (adapts) the native C++ interaces to rust traits, structs, and other types.

#### **3. Build and install the example plugin (rust plugin)**

After building the Rust plugin, you should have a libarary named `libhello_world.so` in the build tree. This is the actual rust plugin. This file and any dependencies that you may have added are required to be in the `LD_LIBRARY_PATH`. To run like normal C++ plugins, we suggest that this be placed into the plugins directory under the ${THUNDER_INSTALL_DIR}, but this is not strictly necessary and is inconvenient during edit, compile, test cycles.

Thunder however, requires that the configuration file for the plugin needs to be installed into ${THUNDER_INSTALL_DIR}/etc/WPEFramework/plugins directory.

**TODO** Check this. Is there a way to configure Thunder to search other directories for plugin config files?


#### **4. Run the HelloWorld Rust plugin**

You may or may not need to update your `LD_LIBRARY_PATH`.

```
export LD_LIBRARY_PATH=${THUNDER_INSTALL_DIR}/usr/lib
${THUNDER_INSTALL_DIR/usr/bin/WPEFramework -c ${THUNDER_INSTALL_DIR}/etc/WPEFramework/config.json
```

#### **5. Test the HelloWorld Rust plugin**

There's a nodejs application in the examples directory that can be used to test out the HelloWorld plugin. That app makes a WebSocket connection to Thunder and repeatedly (1/sec) sends JSON/RPC requests to the plugin and gets "Hell from rust" back. 

```
cd ${THUNDER_ROOT}/thunder_rs/examples/hello_world
npm install ws
node sample_plugin_client.js
```

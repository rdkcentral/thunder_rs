

fn main() {
  let meta_data: libloading::Symbol< Box<dyn Plugin> > 
    = lib.get("thunder_service_metadata")

  // let proto = ... 
  println!("name:{}", meta_data.name);

  let their_plugin = meta_data.create( proto );
}

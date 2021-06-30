#[macro_use]
extern crate rocket;
use rocket::config::Config;
use rocket::figment::Figment;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::ser::{Serialize, SerializeTuple, Serializer};
use std::net::{IpAddr, Ipv4Addr};
use wasmer::{imports, Instance, Module, Store, Value};
use wasmer_wasi::WasiState;

use std::env;

#[get("/")]
fn healthz() -> Status {
    Status::Ok
}

#[get("/")]
fn metrics() -> Status {
    Status::Ok
}

fn handle_wasm(
    wasm_path: &String,
    method_name: &mut String,
    args: Option<&String>,
) -> Box<[Value]> {
    let wasm_bytes = std::fs::read(wasm_path).unwrap();

    // Create a Store.
    let store = Store::default();

    // Compile the Wasm module.
    let module = Module::new(&store, wasm_bytes).unwrap();

    // Check module type (WASI / Original Wasm)
    let has_start_exported = module.exports().functions().find(|x| x.name() == "_start");
    let mut import_object = imports! {};

    if let Some(_) = has_start_exported {
        // Create the `WasiEnv`.
        let mut wasi_env = WasiState::new("_").finalize().unwrap();

        // Then, we get the import object related to our WASI
        // and attach it to the Wasm instance.
        import_object = wasi_env.import_object(&module).unwrap();
    }

    // Create instance.
    let instance = Instance::new(&module, &import_object).unwrap();

    // Retrieve the function.
    let func = instance.exports.get_function(method_name).unwrap();

    // Check calling availability.
    let func_type = func.ty().params();

    // Construct args.
    let mut args_vec = vec![];
    if let Some(args) = args {
        let strs = args.split(",").collect::<Vec<&str>>();
        for (pos, val_type) in func_type.iter().enumerate() {
            let str = strs[pos].trim();
            if str.is_empty() {
                panic!("Insufficient parameters to invoke the function.");
            }
            match val_type {
                wasmer::ValType::I32 => args_vec.push(Value::I32(str.parse::<i32>().unwrap())),
                wasmer::ValType::I64 => args_vec.push(Value::I64(str.parse::<i64>().unwrap())),
                wasmer::ValType::F32 => args_vec.push(Value::F32(str.parse::<f32>().unwrap())),
                wasmer::ValType::F64 => args_vec.push(Value::F64(str.parse::<f64>().unwrap())),
                _ => (),
            }
        }
    }

    // Call funtion and return the result.
    func.call(&args_vec).unwrap()
}

struct MyWasmerValue(Box<[Value]>);
impl Serialize for MyWasmerValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let result_count = self.0.len();
        let mut tup = serializer.serialize_tuple(result_count).unwrap();
        for boxed_val in self.0.iter() {
            match boxed_val.ty() {
                wasmer::ValType::I32 => tup.serialize_element(&boxed_val.i32().unwrap()).unwrap(),
                wasmer::ValType::I64 => tup.serialize_element(&boxed_val.i64().unwrap()).unwrap(),
                wasmer::ValType::F32 => tup.serialize_element(&boxed_val.f32().unwrap()).unwrap(),
                wasmer::ValType::F64 => tup.serialize_element(&boxed_val.f64().unwrap()).unwrap(),
                _ => (),
            }
        }
        tup.end()
    }
}

#[get("/")]
fn test() -> status::Custom<Json<MyWasmerValue>> {
    let wasm_path = format!(
        "{}{}{}",
        env::current_dir().unwrap().to_str().unwrap(),
        "/mods/",
        "hello.wasm"
    );

    // Return result as response.
    status::Custom(
        Status::Ok,
        Json(MyWasmerValue(handle_wasm(
            &wasm_path,
            &mut String::from("_start"),
            None,
        ))),
    )
}

#[post("/", data = "<input>")]
fn handler(input: String) -> status::Custom<Json<MyWasmerValue>> {
    let class_name = env::var("MOD_NAME").unwrap_or(String::from("/mods/hello.wasm")); // File name.
    let method_name = env::var("FUNC_HANDLER").unwrap_or(String::from("_start")); // Function name.
    let _timeout = env::var("FUNC_TIMEOUT").unwrap_or(String::new());
    let _runtime = env::var("FUNC_RUNTIME").unwrap_or(String::new());
    let _memory_limit = env::var("FUNC_MEMORY_LIMIT").unwrap_or(String::new());
    let wasm_path = format!(
        "{}{}{}",
        env::current_dir().unwrap().to_str().unwrap(),
        "/",
        class_name
    );

    // Return result as response.
    status::Custom(
        Status::Ok,
        Json(MyWasmerValue(handle_wasm(
            &wasm_path,
            &mut String::from(method_name),
            Some(&input),
        ))),
    )
}

#[launch]
fn rocket() -> _ {
    let port = env::var("FUNC_PORT");
    let port = match port {
        Ok(v) => v.parse::<u16>().unwrap_or(8080),
        Err(_) => 8080,
    };
    rocket::custom(Figment::from(Config {
        port,
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        ..Config::default()
    }))
    .mount("/healthz", routes![healthz])
    .mount("/metrics", routes![metrics])
    .mount("/test", routes![test])
    .mount("/", routes![handler])
}

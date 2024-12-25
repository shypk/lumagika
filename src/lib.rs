
use magika::{Session, ContentType};
use mlua::{Lua};
use mlua::prelude::*;

// ref: mlua 
// https://github.com/mlua-rs/mlua

// ref: magika
// https://github.com/google/magika/blob/main/rust/lib/src/lib.rs

fn identify_content_label(_: &Lua, content: LuaString) -> LuaResult<String> {
    let ret: &str;
    let _ = match Session::new() {
        Ok(magika) => {
            let result = magika.identify_content_sync(&*content.as_bytes());
            match result {
                Ok(result) => {
                    ret = result.content_type().unwrap_or(ContentType::Unknown).info().label;
                },
                Err(e) => {
                    ret = "Error";
                    println!("Error identifying content: {:?}", e);
                },
            }

        },
        Err(e) => {
            ret = "Error";
            println!("Error identifying content: {:?}", e);
        }
    };

    Ok(ret.to_string())
}

fn identify_content(_: &Lua, content: LuaString) -> LuaResult<(String, String)> {
    let ret: &str = "Unknown";
    return match Session::new() {
        Ok(magika) => {
            let result = magika.identify_content_sync(&*content.as_bytes());
            match result {
                Ok(result) => {
                    let info = result.content_type().unwrap_or(ContentType::Unknown).info();
                    Ok((info.label.to_string(), info.mime_type.to_string()))
                },
                Err(e) => {
                    println!("Error identifying content: {:?}", e);
                    Ok((ret.to_string(), ret.to_string()))
                },
            }

        },
        Err(e) => {
            println!("Error identifying content: {:?}", e);
            Ok((ret.to_string(), ret.to_string()))
        }
    };
}

#[mlua::lua_module]
fn liblumagika(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("identify_content_label", lua.create_function(identify_content_label)?)?;
    exports.set("identify_content", lua.create_function(identify_content)?)?;
    Ok(exports)
}





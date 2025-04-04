use mlua::Lua;

fn main() {
    let lua = Lua::new();

    let script = lua.load(
        r#"
print("print from lua")
io.write("io.write from lua")
        "#,
    );

    println!("println! from rust");

    let _ = script.exec();
}

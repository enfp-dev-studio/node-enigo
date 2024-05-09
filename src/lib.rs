// use neon::prelude::*;

// fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
//     Ok(cx.string("hello node"))
// }

// #[neon::main]
// fn main(mut cx: ModuleContext) -> NeonResult<()> {
//     cx.export_function("hello", hello)?;
//     Ok(())
// }

use neon::prelude::*;
use enigo::{
    Button, Coordinate,
    Direction::{Click},
    Enigo, Key, Keyboard, Mouse, Settings,
    Axis::{Horizontal,Vertical},
};

fn move_mouse(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let x = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let y = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
    let abs_flag = cx.argument::<JsBoolean>(2)?.value(&mut cx) as bool;
    let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo instance");
    // let mut enigo = cx.this::<JsBox<Enigo>>()?;
    let _ = enigo.move_mouse(x, y, if abs_flag { Coordinate::Abs} else {Coordinate::Rel});
    Ok(cx.undefined())
}

fn mouse_click(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let button_str = cx.argument::<JsString>(0)?.value(&mut cx);
    let button = match button_str.as_str() {
        "left" => Button::Left,
        "right" => Button::Right,
        _ => panic!("Unsupported mouse button"),
    };
    let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo instance");
    // let mut enigo = cx.this::<JsBox<Enigo>>()?;
    let _ = enigo.button(button, Click);
    Ok(cx.undefined())
}

fn mouse_scroll(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let length = cx.argument::<JsNumber>(0)?.value(&mut cx) as i32;
    let is_vertical = cx.argument::<JsBoolean>(1)?.value(&mut cx) as bool;
    let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo instance");
    // let mut enigo = cx.this::<JsBox<Enigo>>()?;
    // length: i32, axis: Axis
    let _ = enigo.scroll(length, if is_vertical {Vertical} else {Horizontal});
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("mouse_move", move_mouse)?;
    cx.export_function("mouse_click", mouse_click)?;
    cx.export_function("mouse_scroll", mouse_scroll)?;
    Ok(())
}

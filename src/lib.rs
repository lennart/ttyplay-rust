use wasm_bindgen::prelude::*;

fn param_str(params: &vte::Params) -> String {
    let strs: Vec<_> = params
        .iter()
        .map(|subparams| {
            let subparam_strs: Vec<_> = subparams
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            subparam_strs.join(" : ")
        })
        .collect();
    strs.join(" ; ")
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse_ttyrec_frames(frames: &js_sys::Array) -> Result<js_sys::Array, JsValue> {
    env_logger::init();
    log("[ttyplay] De bug");
    let mut parser = vt100::Parser::new(
        24,
        80,
        0,
        |screen: &vt100::Screen, final_char: char, params: &vte::Params, intermediate: Option<u8>| {
            match intermediate {
                None => {
                    if final_char == 'z' {
                        println!("vt tileset or something @ {:#?}", screen.cursor_position());
                        println!("params: {:#?}", param_str(params))

                    } else {
                        println!("unhandled csi {}", final_char);
                        println!("params: {:#?}", params)
                    }
                },
                Some(b'?') => {
                    println!("unhandled csi '? {}'", final_char);
                    println!("params: {:#?}", params)

                },
                Some(i) => {
                    println!("unhandled csi '{} {}'", i, final_char);
                    println!("params: {:#?}", params)
                }
            }
        }
    );

    let rendered_frames = js_sys::Array::new();
    let mut frame_index = 0;
    for frame in frames.values() {
        let screen = parser.screen().clone();
        let frame = frame?;
        let frame_string = frame.as_string();
        match frame_string {
            None => println!("empty frame!"),
            Some(str) => {
                parser.process(str.as_bytes());
                // log(format!("frame {} processed", frame_index).as_str());
            }
        }

        let rendered_frame_result = String::from_utf8(parser.screen().contents_formatted());
        match rendered_frame_result {
            Ok(rendered_frame) => {
                let rendered_frame_js = &JsValue::from(rendered_frame);
                rendered_frames.push(rendered_frame_js);
            },
            Err(err) => return Err(JsValue::from_str("Failed converting screen contents to UTF8 String"))
        }

        frame_index += 1;
    }

    Ok(rendered_frames)
}

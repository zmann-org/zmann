use nih_plug::prelude::Editor;
use nih_plug::prelude::Param;
use nih_plug_webview::{ http::{ header::CONTENT_TYPE, Response }, HTMLSource, WebViewEditor };
use rust_embed::RustEmbed;
use std::sync::{ atomic::Ordering, Arc };
use serde::Deserialize;
use serde_json::json;
use crate::Presets;
use crate::params::OrchestronParams;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Action {
    Init,
    SetPreset {
        preset: Presets,
    },
    SetGain {
        value: f32,
    },
}

#[derive(RustEmbed)]
#[folder = "$ORCHESTRON_UI_DIST"]
struct WebAssets;

pub(crate) fn create(params: Arc<OrchestronParams>) -> Option<Box<dyn Editor>> {
    let gain_change = params.gain_change.clone();
    let preset_change = params.preset_change.clone();

    let mut editor = WebViewEditor::new(HTMLSource::URL("zmann://localhost/index.html"), (800, 350))
        .with_custom_protocol("zmann".into(), move |request| {
            let path = request.uri().path();
            let mimetype = if path.ends_with(".html") {
                "text/html"
            } else if path.ends_with(".js") {
                "text/javascript"
            } else if path.ends_with(".css") {
                "text/css"
            } else if path.ends_with(".png") {
                "image/png"
            } else {
                "application/octet-stream" // falback, replace with mime_guess
            };

            match <WebAssets as rust_embed::RustEmbed>::get(path.trim_start_matches("/")) {
                Some(content) => {
                    return Response::builder()
                        .header(CONTENT_TYPE, mimetype)
                        .header("Access-Control-Allow-Origin", "*")
                        .body(content.data.to_vec().into())
                        .map_err(Into::into);
                }
                None => {
                    return Response::builder()
                        .header(CONTENT_TYPE, "text/html")
                        .header("Access-Control-Allow-Origin", "*")
                        .body((b"not found" as &[u8]).into())
                        .map_err(Into::into);
                }
            }
        })
        .with_background_color((40, 39, 41, 255))
        .with_developer_mode(true)
        .with_event_loop(move |ctx, setter, _window| {
            while let Ok(value) = ctx.next_event() {
                if let Ok(action) = serde_json::from_value(value.clone()) {
                    match action {
                        Action::SetPreset { preset } => {
                            setter.begin_set_parameter(&params.preset);
                            setter.set_parameter(&params.preset, preset);
                            setter.end_set_parameter(&params.preset);
                        }
                        Action::Init => {
                            let _ = ctx.send_json(
                                json!([{
                                    "type": "preset_change",
                                    "value": params.preset.value().to_string(),
                                }, {
                                    "type": "gain_change",
                                    "value": params.gain.modulated_normalized_value().to_string(),
                                }])
                            );
                        }
                        Action::SetGain { value } => {
                            setter.begin_set_parameter(&params.gain);
                            setter.set_parameter_normalized(&params.gain, value);
                            setter.end_set_parameter(&params.gain);
                        }
                    }
                }
            }

            if preset_change.swap(false, Ordering::Relaxed) {
                let _ = ctx.send_json(
                    json!({
                        "type": "preset_change",
                        "value": params.preset.value().to_string(),
                    })
                );
            }

            if gain_change.swap(false, Ordering::Relaxed) {
                let _ = ctx.send_json(
                    json!({
                        "type": "gain_change",
                        "value": params.gain.modulated_normalized_value().to_string(),
                    })
                );
            }
        });

    #[cfg(windows)]
    {
        editor = editor.with_caption_color(0x00292728).with_browser_accelerator_keys(true);
    }

    Some(Box::new(editor))
}

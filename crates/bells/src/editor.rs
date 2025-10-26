use std::sync::Arc;

use cyma::bus::{Bus, MonoBus};
use cyma::prelude::Oscilloscope;
use cyma::prelude::ValueScaling;
use nih_plug::prelude::{Editor, Enum};
use strum::VariantNames;
use vizia_plug::vizia::prelude::*;
use vizia_plug::widgets::*;
use vizia_plug::{create_vizia_editor, ViziaState, ViziaTheming};

use crate::{BellsParams, Presets};

#[derive(Lens)]
struct Data {
    params: Arc<BellsParams>,
    presets: Vec<&'static str>,
    selected_option: usize,
}

enum EditorEvent {
    SetPreset(usize),
}

impl Model for Data {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|editor_event, _| match editor_event {
            EditorEvent::SetPreset(index) => {
                self.selected_option = *index;

                cx.emit(
                    ParamEvent::SetParameter(&self.params.preset, Presets::from_index(*index))
                        .upcast(),
                );
            }
        });
    }
}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (1000, 800))
}

pub(crate) fn create(
    params: Arc<BellsParams>,
    editor_state: Arc<ViziaState>,
    bus: Arc<MonoBus>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        Data {
            params: params.clone(),
            presets: Presets::VARIANTS.to_vec(),
            selected_option: params.preset.value().to_index(),
        }
        .build(cx);

        bus.subscribe(cx);

        VStack::new(cx, |cx| {
            ComboBox::new(cx, Data::presets, Data::selected_option)
                .on_select(|cx, index| {
                    cx.emit(EditorEvent::SetPreset(index));
                })
                .width(Pixels(100.0));

            Oscilloscope::new(cx, bus.clone(), 4.0, (-1.0, 1.0), ValueScaling::Linear)
                .color(Color::rgb(120, 120, 120));
        })
        .alignment(Alignment::TopCenter);
    })
}

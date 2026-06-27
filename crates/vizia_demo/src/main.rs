#[allow(unused)]
use vizia::prelude::*;

#[derive(Clone, Lens)]
struct ComboBoxState {
    options: Vec<&'static str>,
    selected_option: usize,
}

pub enum ComboBoxEvent {
    SetOption(usize),
}

impl Model for ComboBoxState {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            ComboBoxEvent::SetOption(index) => {
                self.selected_option = *index;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("style.css"))
            .expect("Failed to add stylesheet");

        ComboBoxState {
            options: vec![
                "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            ],

            selected_option: 0,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            ComboBox::new(cx, ComboBoxState::options, ComboBoxState::selected_option)
                .on_select(|cx, index| cx.emit(ComboBoxEvent::SetOption(index)))
                .width(Pixels(100.0));
        })
        .size(Pixels(200.0));
    })
    // .ignore_default_theme()
    .run()
}

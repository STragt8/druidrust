
use druid::widget::{Label, Flex, Button};
use druid::{AppLauncher, LocalizedString, Widget, WindowDesc, Env, Data};

#[derive(Clone, Data)]
struct Number {
    number: u32
}

fn main() {
    let window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("My App"))
        .window_size((400.0, 400.0));
    let launcher = AppLauncher::with_window(window);
    launcher.launch(Number { number: 0 }).expect("Failed");
}


fn ui_builder() -> impl Widget<Number> {
    let label = Label::new(|data: &Number, _: &Env| format!("Counter: {}", data.number));
    let button = Button::new("+")
        .on_click(|_ctx, data: &mut Number, _env|data.number+=1);
    let button1 = Button::new("-")
        .on_click(|_ctx, data: &mut Number, _env|data.number-=1);


    Flex::column()
        .with_child(label)
        .with_spacer(20.0)
        .with_child(button)
        .with_spacer(20.0)
        .with_child(button1)
}
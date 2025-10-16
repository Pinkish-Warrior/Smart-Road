use druid::{
    commands, keyboard_types::Key, widget::Label, AppDelegate, AppLauncher, Command, DelegateCtx,
    Env, Event, Handled, Point, Screen, Target, Widget, WidgetExt, WindowDesc, WindowId,
};

pub fn show(s: &str) {
    let screen = Screen::get_monitors()[0].virtual_rect();
    let x = screen.x0 + (screen.x1 - screen.x0) / 2.0;
    let y = screen.y0 + (screen.y1 - screen.y0) / 2.0;

    let main_window = WindowDesc::new(ui_builder(s))
        .window_size((600.0, 400.0))
        .title("Smart Road")
        .set_position(Point::new(x - 300.0, y - 200.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .launch(())
        .expect("Launch failed");
}

struct Delegate;

impl AppDelegate<()> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut (),
        _env: &Env,
    ) -> Handled {
        if cmd.is(commands::CLOSE_WINDOW) {
            ctx.submit_command(commands::QUIT_APP);
            true.into()
        } else {
            false.into()
        }
    }

    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        _window_id: WindowId,
        event: Event,
        _data: &mut (),
        _env: &Env,
    ) -> Option<Event> {
        match event {
            Event::KeyDown(ref key_event) => {
                if key_event.key == Key::Escape {
                    ctx.submit_command(commands::QUIT_APP);
                }
            }
            _ => {}
        }
        Some(event)
    }
}

fn ui_builder(s: &str) -> impl Widget<()> {
    Label::new(s).center().on_click(|ctx, _, _| {
        ctx.submit_command(commands::CLOSE_WINDOW.to(ctx.window_id()));
    })
}

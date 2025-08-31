mod app;
mod game_state;
mod renderer;

use app::App;

fn main() -> Result<(), String> {
    let mut app = App::new().map_err(|e| e.to_string())?;
    app.run()
}
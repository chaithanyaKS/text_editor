use editor::Editor;
mod editor;
mod terminal;
pub use editor::Position;
pub use terminal::Terminal;
fn main() {
    Editor::default().run();
}

fn main() {
    let rc_path = "mario-minesweeper.rc";
    if std::path::Path::new(rc_path).exists() {
        let content = std::fs::read_to_string(rc_path).unwrap_or_default();
        if !content.trim().is_empty() {
            embed_resource::compile(rc_path, embed_resource::NONE);
        }
    }
}

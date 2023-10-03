use windres::Build;

fn main() {
    Build::new()
        .compile("wiresx_dashboard_companion.rc")
        .unwrap();
}

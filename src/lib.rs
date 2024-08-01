pub mod css;
pub mod submenu;
pub mod ext;
pub mod swkbd;

#[skyline::main(name = "css_controls")]
pub fn main() {
    css::install();
}

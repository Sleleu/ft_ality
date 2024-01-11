mod parsing;
mod display;
mod sdl_init;
mod combos;
mod handle_args;

fn main() {
    let args = handle_args::get_args();
    handle_args::check_arg_len(args.len());
    let str_file = handle_args::get_file(&args[1]);
    parsing::parse_file(&str_file);
    let (keymap, combos) = parsing::parse_file(&str_file);
    display::display_banner();
    display::display_keymap(&keymap);
    sdl_init::init_sdl(keymap, combos);
}

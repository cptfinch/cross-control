use xkbcommon::xkb;

#[test]
fn test_xkb_init() {
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    println!("Context: {:?}", context);
    
    let keymap = xkb::Keymap::new_from_names(
        &context,
        None,
        None,
        None,
        None,
        None,
        xkb::KEYMAP_COMPILE_NO_FLAGS
    );
    println!("Keymap: {:?}", keymap);
} 
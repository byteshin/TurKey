#[allow(dead_code)]
#[allow(unused_imports)]
mod bindings {
  ::windows::include_bindings!();
}

#[macro_use]
extern crate error_chain;
mod errors {
  error_chain! {
    foreign_links {
      Io(::std::io::Error);
    }
  }
}

mod export;
mod hotkey;
mod keyboard;
mod mouse;
mod window;

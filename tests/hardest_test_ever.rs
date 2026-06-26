use std::io::{self, Write};

static mut X: u128 = 100;

#[test]
#[should_panic]
#[allow(unconditional_recursion)]
fn main() {
    unsafe {
        X = 7;
        loop {
        match X {
            100 => {
                fn main() {
                    print!("{}\n", 100)
                }
                main();
            }
            _ => {
                fn main() {
                    panic!("ur mom")
                }
                main();
            break;
            }
        }

        (|| {
            let new_x = &raw const X as *const u128 as *const std::sync::Arc<std::sync::Mutex<&'static str>>;
        
            let meow_mix: &std::sync::Arc<std::sync::Mutex<&'static str>> = &*new_x;

    write!(io::stdout().lock(), "{}", meow_mix.lock().unwrap()).unwrap();
        })()
    }
    
    main();
    

                std::process::exit(0);
    }
}
    

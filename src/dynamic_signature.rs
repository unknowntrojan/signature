use core::fmt;

use module;

///
/// A signature that is evaluated at run-time.
///
/// # Examples
/// ```
/// # use signature::Signature;
/// # use signature::dynamic_signature;
/// let mut sig = dynamic_signature!("user32.dll", 0xE9, _, _, _);
/// let addr: *mut u8 = sig.get().unwrap();
/// ```
///
#[derive(Debug)]
pub struct DynamicSignature {
    addr: Option<*mut u8>,
    module: String,
    pattern: findpattern::Pattern,
}

impl DynamicSignature {
    pub fn new(module: String, pattern: findpattern::Pattern) -> Self {
        DynamicSignature {
            addr: None,
            module,
            pattern,
        }
    }
}

///
/// Creates a signature that is evaluated at run-time.
///
/// # Examples
/// ```
/// # use signature::Signature;
/// # use signature::dynamic_signature;
/// let mut sig = dynamic_signature!("user32.dll", 0xE9, _, _, _);
/// let addr: *mut u8 = sig.get().unwrap();
/// ```
///
#[macro_export]
macro_rules! dynamic_signature {
    ($module:expr, $($elem:tt),+) => {
        signature::dynamic_signature::DynamicSignature::new(String::from($module), vec![$(dynamic_signature!(@el $elem)),+])
    };
    (@el $v:expr) => {
        Some($v)
    };
    (@el $v:tt) => {
        None
    };
}

impl fmt::Display for DynamicSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // dont bully me, i cba making this fancy
        write!(f, "Dynamic [ ").and_then(|_| {
            self.pattern
                .iter()
                .fold(Ok(()), |result, pat| {
                    result.and_then(|_| {
                        write!(
                            f,
                            "{} ",
                            match pat {
                                Some(x) => format!("{:02X}", x),
                                None => "?".into(),
                            }
                        )
                    })
                })
                .and_then(|_| {
                    writeln!(
                        f,
                        "] @ {} => {}",
                        self.module,
                        match self.addr {
                            Some(x) => format!("{:p}", x),
                            None => "[NOT LOADED]".into(),
                        }
                    )
                })
        })
    }
}

impl crate::Signature for DynamicSignature {
    fn get(&mut self) -> Option<*mut u8> {
        if let Some(x) = self.addr {
            return Some(x);
        }

        let region = module::get_dynamic(self.module.to_owned(), true)?;

        unsafe {
            let memory = std::slice::from_raw_parts(region.0, region.1);

            let offset = findpattern::find_pattern(memory, &self.pattern);

            self.addr = offset.map(|x| (region.0 as usize + x) as *mut u8)
        }

        self.addr
    }
}

use core::fmt;

use module;

///
/// A signature that is evaluated at compile-time by a proc macro.
///
/// # Examples
/// ```
/// # use signature::Signature;
/// # use signature::static_signature;
/// let mut sig = static_signature!("user32.dll", 0xE9, _, _, _);
/// let addr: *mut u8 = sig.get().unwrap();
/// ```
///
#[derive(Debug)]
pub struct StaticSignature {
    addr: Option<*mut u8>,
    module: String,
    offset: usize,
    sanity: u8,
}

impl StaticSignature {
    pub fn new(a: (String, usize, u8)) -> Self {
        StaticSignature {
            addr: None,
            module: a.0,
            offset: a.1,
            sanity: a.2,
        }
    }
}

///
/// Creates a signature that is evaluated at compile-time.
///
/// # Examples
/// ```
/// # use signature::Signature;
/// # use signature::static_signature;
/// let mut sig = static_signature!("user32.dll", 0xE9, _, _, _);
/// let addr: *mut u8 = sig.get().unwrap();
/// ```
///
#[macro_export]
macro_rules! static_signature {
    ($($x:tt)*) => {
        signature::static_signature::StaticSignature::new(signature::signature_macro::static_macro!($($x)*))
    }
}

impl fmt::Display for StaticSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Static [ {:#04X}/{:02X} ] @ {} => {}",
            self.offset,
            self.sanity,
            self.module,
            match self.addr {
                Some(x) => format!("{:p}", x),
                None => "[NOT LOADED]".into(),
            }
        )
    }
}

impl crate::Signature for StaticSignature {
    fn get(&mut self) -> Option<*mut u8> {
        if let Some(x) = self.addr {
            return Some(x);
        }

        let region = module::get_dynamic(self.module.to_owned(), true)?;

        let addr = (region.0 as usize + self.offset) as *mut u8;

        unsafe {
            self.addr = match *addr == self.sanity {
                true => Some(addr),
                false => None,
            }
        }

        self.addr
    }
}

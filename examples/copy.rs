use rand::Rng;
use std::fmt;

// #[derive(Debug)]

enum Error {
    Description(&'static str),
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt::Debug::fmt(&self.repr, f)
        match *self {
            Self::Description(code) => fmt
                .write_str(format!("bla: {}", code).as_str())
                // .debug_struct("Os")
                // .field("code", &code)
                // .finish(),
        }
    }
}


#[async_std::main]
async fn main() -> Result<(), Error> {
    let i = check_luck().expect("checking my luck");
    println!("i: {}", i);
    Ok(())
}

fn check_luck() -> Result<i64, Error> {
    Ok(maybe_bad()?)
}

fn maybe_bad() -> Result<i64, Error> {
    let mut rng = rand::thread_rng();
    if rng.gen() {
        return Err(Error::Description("bad luck"))
    }

    Ok(42)
}


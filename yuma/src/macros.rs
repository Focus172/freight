#[macro_export]
macro_rules! pkg {
    ($name:expr) => {
        Into::<$crate::prelude::Pkg>::into($name)
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn compiles() {
        let _ = pkg!("test");
    }
}

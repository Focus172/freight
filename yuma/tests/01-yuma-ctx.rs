use std::time::Duration;

use yuma::prelude::*;

#[test]
fn implicit_drop_callback() {
    let (tx, rx) = std::sync::mpsc::channel();
    {
        let mut ctx = ctx();
        ctx.skip_cache();

        ctx.schedule("test", move || {
            assert!(tx.send(()).is_ok());
            Ok(())
        });
    }
    assert!(rx.recv_timeout(Duration::from_millis(500)).is_ok());
}

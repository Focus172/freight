use yuma::prelude::*;

#[test]
fn implicit_drop_callback() {
    let (tx, rx) = std::sync::mpsc::channel();

    {
        let mut ctx = YumaCtx::stub();

        ctx.schedule("test", move || {
            assert!(tx.send(()).is_ok());
            Ok(())
        });
    }

    assert!(rx.recv().is_ok());
}

#[test]
fn feature() {
    {
        let mut ctx = YumaCtx::stub();

        let pkgs = Pkgs::new(
            yuma::deriv::packager::PackagerType::Fake,
            vec![].into_iter(),
        );

        ctx.add(pkgs)
    }
}

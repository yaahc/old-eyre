use std::borrow::Cow;
fn can_catch_strings() -> eyre::Result<()> {
    Err("watch me catch this string")?;

    Ok(())
}

fn can_catch_cows() -> eyre::Result<()> {
    Err(Cow::Borrowed("holy crap was that a cow??? mooooo"))?;

    Ok(())
}

#[test]
fn can_catch_all_the_things() {
    can_catch_strings()
        .map_err(|e| println!("{:?}", e))
        .unwrap_err();
    can_catch_cows()
        .map_err(|e| println!("{:?}", e))
        .unwrap_err();
}

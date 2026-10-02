//! Rendert das Testbild und den Droste-Effekt als PNG.
//!
//! `cargo run -p droste --example render --release -- [Zielverzeichnis]`

fn main() -> std::io::Result<()> {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    let s = 4.0;
    let src = droste::test_pattern(1200, s);
    src.save_png(format!("{dir}/droste_quelle.png"))?;
    droste::droste(&src, 900, 900, s, 1.0, false).save_png(format!("{dir}/droste_gerade.png"))?;
    droste::droste(&src, 900, 900, s, 1.0, true).save_png(format!("{dir}/droste_spirale.png"))?;
    println!("geschrieben nach {dir}");
    Ok(())
}

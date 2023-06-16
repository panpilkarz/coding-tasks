use std::io::Write;
use rot13::Rot13Writer;

fn main() {
    let mut content = Vec::<u8>::default();

    {
        let mut buff = Rot13Writer::new(&mut content);
        // fixme: use write_all() instead
        buff.write(b"Lbh penpxrq zl fhcre qvssvphyg pbqvat punyyratr... pbqr vf ddommNst").unwrap();
        buff.flush().expect("Failed to flush");
    }

    println!(
        "result: {:?}",
        content.iter().map(|x| *x as char).collect::<String>()
    );
}
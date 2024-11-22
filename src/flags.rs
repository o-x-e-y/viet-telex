use std::path::PathBuf;

xflags::xflags! {
    cmd viet-telex {
        required viet: PathBuf
        required -o, --out out: PathBuf
        optional --overwrite
    }
}

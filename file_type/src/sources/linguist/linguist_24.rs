use crate::format::FileFormat;

pub(crate) const LINGUIST_24: FileFormat = FileFormat {
    id: 24,
    puid: "linguist/24",
    name: "Assembly",
    extensions: &["a51", "asm", "i", "inc", "nas", "nasm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

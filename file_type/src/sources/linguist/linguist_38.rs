use crate::format::FileFormat;

pub(crate) const LINGUIST_38: FileFormat = FileFormat {
    id: 38,
    puid: "linguist/38",
    name: "Brainfuck",
    extensions: &["b", "bf"],
    media_types: &["text/x-brainfuck"],
    internal_signatures: &[],
    related_formats: &[],
};

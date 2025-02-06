use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_271: FileFormat = FileFormat {
    id: 271,
    source_type: SourceType::Linguist,
    name: "Pawn",
    extensions: &["inc", "pwn", "sma"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

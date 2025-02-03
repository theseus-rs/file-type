use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_270184138: FileFormat = FileFormat {
    id: 270_184_138,
    source_type: SourceType::Linguist,
    name: "Cadence",
    extensions: &["cdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

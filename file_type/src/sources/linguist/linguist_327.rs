use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_327: FileFormat = FileFormat {
    id: 327,
    source_type: SourceType::Linguist,
    name: "Rust",
    extensions: &["rs", "rs.in"],
    media_types: &["text/x-rustsrc"],
    internal_signatures: &[],
    related_formats: &[],
};

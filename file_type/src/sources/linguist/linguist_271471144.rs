use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_271471144: FileFormat = FileFormat {
    id: 271_471_144,
    source_type: SourceType::Linguist,
    name: "Sway",
    extensions: &["sw"],
    media_types: &["text/x-rustsrc"],
    internal_signatures: &[],
    related_formats: &[],
};

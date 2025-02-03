use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_564743864: FileFormat = FileFormat {
    id: 564_743_864,
    source_type: SourceType::Linguist,
    name: "Modula-3",
    extensions: &["i3", "ig", "m3", "mg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

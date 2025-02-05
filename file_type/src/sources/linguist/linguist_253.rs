use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_253: FileFormat = FileFormat {
    id: 253,
    source_type: SourceType::Linguist,
    name: "Nu",
    extensions: &["nu"],
    media_types: &["text/x-scheme"],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_203: FileFormat = FileFormat {
    id: 203,
    source_type: SourceType::Linguist,
    name: "Linux Kernel Module",
    extensions: &["mod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

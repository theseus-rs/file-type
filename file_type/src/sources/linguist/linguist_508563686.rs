use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_508563686: FileFormat = FileFormat {
    id: 508_563_686,
    source_type: SourceType::Linguist,
    name: "Vim Help File",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

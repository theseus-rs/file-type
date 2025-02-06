use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_211: FileFormat = FileFormat {
    id: 211,
    source_type: SourceType::Linguist,
    name: "LookML",
    extensions: &["lkml", "lookml"],
    media_types: &["text/x-yaml"],
    signatures: &[],
    related_formats: &[],
};

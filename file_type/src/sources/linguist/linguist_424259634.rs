use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_424259634: FileFormat = FileFormat {
    id: 424_259_634,
    source_type: SourceType::Linguist,
    name: "CodeQL",
    extensions: &["ql", "qll"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_352: FileFormat = FileFormat {
    id: 352,
    source_type: SourceType::Linguist,
    name: "Smalltalk",
    extensions: &["cs", "st"],
    media_types: &["text/x-stsrc"],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_899227493: FileFormat = FileFormat {
    id: 899_227_493,
    source_type: SourceType::Linguist,
    name: "RBS",
    extensions: &["rbs"],
    media_types: &["text/x-ruby"],
    signatures: &[],
    related_formats: &[],
};

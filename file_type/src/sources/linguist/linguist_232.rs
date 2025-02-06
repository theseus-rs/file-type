use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_232: FileFormat = FileFormat {
    id: 232,
    source_type: SourceType::Linguist,
    name: "Mirah",
    extensions: &["druby", "duby", "mirah"],
    media_types: &["text/x-ruby"],
    signatures: &[],
    related_formats: &[],
};

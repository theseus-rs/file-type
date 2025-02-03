use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_389477596: FileFormat = FileFormat {
    id: 389_477_596,
    source_type: SourceType::Linguist,
    name: "AngelScript",
    extensions: &["angelscript", "as"],
    media_types: &["text/x-c++src"],
    internal_signatures: &[],
    related_formats: &[],
};

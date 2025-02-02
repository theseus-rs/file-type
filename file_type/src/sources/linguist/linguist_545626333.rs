use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_545626333: FileFormat = FileFormat {
    id: 545_626_333,
    source_type: SourceType::Linguist,
    name: "Beef",
    extensions: &["bf"],
    media_types: &["text/x-csharp"],
    internal_signatures: &[],
    related_formats: &[],
};

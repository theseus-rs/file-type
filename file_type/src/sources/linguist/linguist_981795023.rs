use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_981795023: FileFormat = FileFormat {
    id: 981_795_023,
    source_type: SourceType::Linguist,
    name: "TextMate Properties",
    extensions: &[],
    media_types: &["text/x-properties"],
    internal_signatures: &[],
    related_formats: &[],
};

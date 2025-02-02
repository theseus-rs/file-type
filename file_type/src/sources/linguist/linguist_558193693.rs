use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_558193693: FileFormat = FileFormat {
    id: 558_193_693,
    source_type: SourceType::Linguist,
    name: "Qt Script",
    extensions: &["qs"],
    media_types: &["text/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};

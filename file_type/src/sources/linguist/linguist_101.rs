use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_101: FileFormat = FileFormat {
    id: 101,
    source_type: SourceType::Linguist,
    name: "Elm",
    extensions: &["elm"],
    media_types: &["text/x-elm"],
    internal_signatures: &[],
    related_formats: &[],
};

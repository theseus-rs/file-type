use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_371: FileFormat = FileFormat {
    id: 371,
    source_type: SourceType::Linguist,
    name: "Terra",
    extensions: &["t"],
    media_types: &["text/x-lua"],
    internal_signatures: &[],
    related_formats: &[],
};

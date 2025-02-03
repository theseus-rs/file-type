use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_112: FileFormat = FileFormat {
    id: 112,
    source_type: SourceType::Linguist,
    name: "Filterscript",
    extensions: &["fs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

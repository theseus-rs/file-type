use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_359: FileFormat = FileFormat {
    id: 359,
    source_type: SourceType::Linguist,
    name: "Stylus",
    extensions: &["styl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

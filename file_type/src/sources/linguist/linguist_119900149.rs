use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_119900149: FileFormat = FileFormat {
    id: 119_900_149,
    source_type: SourceType::Linguist,
    name: "Slint",
    extensions: &["slint"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

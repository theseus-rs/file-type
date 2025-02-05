use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_397: FileFormat = FileFormat {
    id: 397,
    source_type: SourceType::Linguist,
    name: "X10",
    extensions: &["x10"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

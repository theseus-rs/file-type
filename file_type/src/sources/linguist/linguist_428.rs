use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_428: FileFormat = FileFormat {
    id: 428,
    source_type: SourceType::Linguist,
    name: "Python console",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

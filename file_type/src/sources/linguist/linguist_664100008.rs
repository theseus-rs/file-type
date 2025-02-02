use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_664100008: FileFormat = FileFormat {
    id: 664_100_008,
    source_type: SourceType::Linguist,
    name: "OMNeT++ MSG",
    extensions: &["msg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_328: FileFormat = FileFormat {
    id: 328,
    source_type: SourceType::Linguist,
    name: "SAS",
    extensions: &["sas"],
    media_types: &["text/x-sas"],
    internal_signatures: &[],
    related_formats: &[],
};

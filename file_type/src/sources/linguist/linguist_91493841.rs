use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_91493841: FileFormat = FileFormat {
    id: 91_493_841,
    source_type: SourceType::Linguist,
    name: "Clarity",
    extensions: &["clar"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

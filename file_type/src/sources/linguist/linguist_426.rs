use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_426: FileFormat = FileFormat {
    id: 426,
    source_type: SourceType::Linguist,
    name: "MQL4",
    extensions: &["mq4", "mqh"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

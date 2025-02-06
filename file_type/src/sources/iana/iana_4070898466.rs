use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4070898466: FileFormat = FileFormat {
    id: 4_070_898_466,
    source_type: SourceType::Iana,
    name: "vnd.mozilla.apng",
    extensions: &[],
    media_types: &["image/vnd.mozilla.apng"],
    signatures: &[],
    related_formats: &[],
};

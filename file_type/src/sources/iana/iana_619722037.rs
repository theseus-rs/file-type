use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_619722037: FileFormat = FileFormat {
    id: 619_722_037,
    source_type: SourceType::Iana,
    name: "hmpg",
    extensions: &[],
    media_types: &["haptics/hmpg"],
    signatures: &[],
    related_formats: &[],
};

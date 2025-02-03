use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1115896451: FileFormat = FileFormat {
    id: 1_115_896_451,
    source_type: SourceType::Iana,
    name: "vnd.keyman.kmp+zip",
    extensions: &[],
    media_types: &["application/vnd.keyman.kmp+zip"],
    internal_signatures: &[],
    related_formats: &[],
};

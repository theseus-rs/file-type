use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1936666943: FileFormat = FileFormat {
    id: 1_936_666_943,
    source_type: SourceType::Iana,
    name: "vnd.kodak-descriptor",
    extensions: &[],
    media_types: &["application/vnd.kodak-descriptor"],
    signatures: &[],
    related_formats: &[],
};

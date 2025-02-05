use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2358701609: FileFormat = FileFormat {
    id: 2_358_701_609,
    source_type: SourceType::Iana,
    name: "its+xml",
    extensions: &[],
    media_types: &["application/its+xml"],
    signatures: &[],
    related_formats: &[],
};

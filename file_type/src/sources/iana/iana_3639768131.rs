use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3639768131: FileFormat = FileFormat {
    id: 3_639_768_131,
    source_type: SourceType::Iana,
    name: "mbms-msk-response+xml",
    extensions: &[],
    media_types: &["application/mbms-msk-response+xml"],
    signatures: &[],
    related_formats: &[],
};

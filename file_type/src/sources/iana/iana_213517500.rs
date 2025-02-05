use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_213517500: FileFormat = FileFormat {
    id: 213_517_500,
    source_type: SourceType::Iana,
    name: "vnd.mophun.certificate",
    extensions: &[],
    media_types: &["application/vnd.mophun.certificate"],
    signatures: &[],
    related_formats: &[],
};

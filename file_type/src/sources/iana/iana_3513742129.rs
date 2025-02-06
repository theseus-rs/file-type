use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3513742129: FileFormat = FileFormat {
    id: 3_513_742_129,
    source_type: SourceType::Iana,
    name: "vnd.japannet-directory-service",
    extensions: &[],
    media_types: &["application/vnd.japannet-directory-service"],
    signatures: &[],
    related_formats: &[],
};

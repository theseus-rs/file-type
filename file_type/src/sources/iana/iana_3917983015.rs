use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3917983015: FileFormat = FileFormat {
    id: 3_917_983_015,
    source_type: SourceType::Iana,
    name: "vnd.dolby.mobile.2",
    extensions: &[],
    media_types: &["application/vnd.dolby.mobile.2"],
    signatures: &[],
    related_formats: &[],
};

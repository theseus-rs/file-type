use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1088321248: FileFormat = FileFormat {
    id: 1_088_321_248,
    source_type: SourceType::Iana,
    name: "vnd.openvpi.dspx+json",
    extensions: &[],
    media_types: &["application/vnd.openvpi.dspx+json"],
    signatures: &[],
    related_formats: &[],
};

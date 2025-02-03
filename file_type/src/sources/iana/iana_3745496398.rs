use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3745496398: FileFormat = FileFormat {
    id: 3_745_496_398,
    source_type: SourceType::Iana,
    name: "vnd.sss-dtf",
    extensions: &[],
    media_types: &["application/vnd.sss-dtf"],
    internal_signatures: &[],
    related_formats: &[],
};

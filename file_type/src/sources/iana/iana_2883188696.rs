use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2883188696: FileFormat = FileFormat {
    id: 2_883_188_696,
    source_type: SourceType::Iana,
    name: "activemessage",
    extensions: &[],
    media_types: &["application/activemessage"],
    signatures: &[],
    related_formats: &[],
};

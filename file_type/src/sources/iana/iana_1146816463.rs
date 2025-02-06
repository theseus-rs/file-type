use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1146816463: FileFormat = FileFormat {
    id: 1_146_816_463,
    source_type: SourceType::Iana,
    name: "vnd.globalplatform.card-content-mgt",
    extensions: &[],
    media_types: &["application/vnd.globalplatform.card-content-mgt"],
    signatures: &[],
    related_formats: &[],
};

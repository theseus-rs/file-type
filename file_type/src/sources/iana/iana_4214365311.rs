use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4214365311: FileFormat = FileFormat {
    id: 4_214_365_311,
    source_type: SourceType::Iana,
    name: "vnd.wfa.wsc",
    extensions: &[],
    media_types: &["message/vnd.wfa.wsc"],
    signatures: &[],
    related_formats: &[],
};

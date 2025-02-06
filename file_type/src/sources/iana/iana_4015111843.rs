use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4015111843: FileFormat = FileFormat {
    id: 4_015_111_843,
    source_type: SourceType::Iana,
    name: "vnd.ms-PrintSchemaTicket+xml",
    extensions: &[],
    media_types: &["application/vnd.ms-PrintSchemaTicket+xml"],
    signatures: &[],
    related_formats: &[],
};

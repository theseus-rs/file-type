use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4015111843: FileType = FileType {
    file_format: &FileFormat {
        id: 4_015_111_843,
        source_type: SourceType::Iana,
        name: "vnd.ms-PrintSchemaTicket+xml",
        extensions: &[],
        media_types: &["application/vnd.ms-PrintSchemaTicket+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

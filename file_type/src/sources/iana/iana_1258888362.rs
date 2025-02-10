use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1258888362: FileType = FileType {
    file_format: &FileFormat {
        id: 1_258_888_362,
        source_type: SourceType::Iana,
        name: "vnd.radisys.msml-audit-stream+xml",
        extensions: &[],
        media_types: &["application/vnd.radisys.msml-audit-stream+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

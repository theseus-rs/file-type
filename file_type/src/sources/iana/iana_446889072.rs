use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_446889072: FileType = FileType {
    file_format: &FileFormat {
        id: 446_889_072,
        source_type: SourceType::Iana,
        name: "dns",
        extensions: &[],
        media_types: &["application/dns"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_975001307: FileType = FileType {
    file_format: &FileFormat {
        id: 975_001_307,
        source_type: SourceType::Iana,
        name: "vnd.oma.poc.groups+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.poc.groups+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

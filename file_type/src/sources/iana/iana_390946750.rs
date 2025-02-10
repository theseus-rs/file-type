use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_390946750: FileType = FileType {
    file_format: &FileFormat {
        id: 390_946_750,
        source_type: SourceType::Iana,
        name: "load-control+xml",
        extensions: &[],
        media_types: &["application/load-control+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

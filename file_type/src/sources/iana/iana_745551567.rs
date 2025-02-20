use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_745551567: FileType = FileType {
    file_format: &FileFormat {
        id: 745_551_567,
        source_type: SourceType::Iana,
        name: "vnd.restful+json",
        extensions: &[],
        media_types: &["application/vnd.restful+json"],
        signatures: &[],
        related_formats: &[],
    },
};

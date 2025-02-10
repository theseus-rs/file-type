use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2123153113: FileType = FileType {
    file_format: &FileFormat {
        id: 2_123_153_113,
        source_type: SourceType::Iana,
        name: "vnd.ecowin.filerequest",
        extensions: &[],
        media_types: &["application/vnd.ecowin.filerequest"],
        signatures: &[],
        related_formats: &[],
    },
};

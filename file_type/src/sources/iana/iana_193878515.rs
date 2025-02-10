use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_193878515: FileType = FileType {
    file_format: &FileFormat {
        id: 193_878_515,
        source_type: SourceType::Iana,
        name: "sdp",
        extensions: &[],
        media_types: &["application/sdp"],
        signatures: &[],
        related_formats: &[],
    },
};

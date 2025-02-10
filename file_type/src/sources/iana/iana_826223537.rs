use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_826223537: FileType = FileType {
    file_format: &FileFormat {
        id: 826_223_537,
        source_type: SourceType::Iana,
        name: "vnd.font-fontforge-sfd",
        extensions: &[],
        media_types: &["application/vnd.font-fontforge-sfd"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_769171922: FileType = FileType {
    file_format: &FileFormat {
        id: 769_171_922,
        source_type: SourceType::Iana,
        name: "vnd.ascii-art",
        extensions: &[],
        media_types: &["text/vnd.ascii-art"],
        signatures: &[],
        related_formats: &[],
    },
};

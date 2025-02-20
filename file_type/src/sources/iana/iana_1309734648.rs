use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1309734648: FileType = FileType {
    file_format: &FileFormat {
        id: 1_309_734_648,
        source_type: SourceType::Iana,
        name: "vnd.acm.chatbot+json",
        extensions: &[],
        media_types: &["application/vnd.acm.chatbot+json"],
        signatures: &[],
        related_formats: &[],
    },
};

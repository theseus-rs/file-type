use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1626156157: FileType = FileType {
    file_format: &FileFormat {
        id: 1_626_156_157,
        source_type: SourceType::Iana,
        name: "vnd.zoho-presentation.show",
        extensions: &[],
        media_types: &["application/vnd.zoho-presentation.show"],
        signatures: &[],
        related_formats: &[],
    },
};

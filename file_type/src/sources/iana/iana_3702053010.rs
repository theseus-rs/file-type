use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3702053010: FileType = FileType {
    file_format: &FileFormat {
        id: 3_702_053_010,
        source_type: SourceType::Iana,
        name: "vnd.ctct.ws+xml",
        extensions: &[],
        media_types: &["application/vnd.ctct.ws+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

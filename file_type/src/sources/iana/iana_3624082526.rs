use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3624082526: FileType = FileType {
    file_format: &FileFormat {
        id: 3_624_082_526,
        source_type: SourceType::Iana,
        name: "vnd.oipf.contentaccessdownload+xml",
        extensions: &[],
        media_types: &["application/vnd.oipf.contentaccessdownload+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

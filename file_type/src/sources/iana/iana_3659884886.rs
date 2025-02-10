use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3659884886: FileType = FileType {
    file_format: &FileFormat {
        id: 3_659_884_886,
        source_type: SourceType::Iana,
        name: "vnd.bary",
        extensions: &[],
        media_types: &["model/vnd.bary"],
        signatures: &[],
        related_formats: &[],
    },
};

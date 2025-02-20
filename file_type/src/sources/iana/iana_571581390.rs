use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_571581390: FileType = FileType {
    file_format: &FileFormat {
        id: 571_581_390,
        source_type: SourceType::Iana,
        name: "vnd.accpac.simply.imp",
        extensions: &[],
        media_types: &["application/vnd.accpac.simply.imp"],
        signatures: &[],
        related_formats: &[],
    },
};

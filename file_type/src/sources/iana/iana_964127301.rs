use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_964127301: FileType = FileType {
    file_format: &FileFormat {
        id: 964_127_301,
        source_type: SourceType::Iana,
        name: "pskc+xml",
        extensions: &[],
        media_types: &["application/pskc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

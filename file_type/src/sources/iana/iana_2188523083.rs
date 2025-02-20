use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2188523083: FileType = FileType {
    file_format: &FileFormat {
        id: 2_188_523_083,
        source_type: SourceType::Iana,
        name: "vnd.oma-scws-config",
        extensions: &[],
        media_types: &["application/vnd.oma-scws-config"],
        signatures: &[],
        related_formats: &[],
    },
};

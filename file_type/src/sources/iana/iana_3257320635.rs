use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3257320635: FileType = FileType {
    file_format: &FileFormat {
        id: 3_257_320_635,
        source_type: SourceType::Iana,
        name: "vnd.debian.binary-package",
        extensions: &[],
        media_types: &["application/vnd.debian.binary-package"],
        signatures: &[],
        related_formats: &[],
    },
};

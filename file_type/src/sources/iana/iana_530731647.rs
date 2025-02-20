use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_530731647: FileType = FileType {
    file_format: &FileFormat {
        id: 530_731_647,
        source_type: SourceType::Iana,
        name: "atomdeleted+xml",
        extensions: &[],
        media_types: &["application/atomdeleted+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

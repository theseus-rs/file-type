use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_484148083: FileType = FileType {
    file_format: &FileFormat {
        id: 484_148_083,
        source_type: SourceType::Iana,
        name: "vnd.muvee.style",
        extensions: &[],
        media_types: &["application/vnd.muvee.style"],
        signatures: &[],
        related_formats: &[],
    },
};

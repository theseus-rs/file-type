use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2723193037: FileType = FileType {
    file_format: &FileFormat {
        id: 2_723_193_037,
        source_type: SourceType::Iana,
        name: "vnd.kde.kformula",
        extensions: &[],
        media_types: &["application/vnd.kde.kformula"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_348215381: FileType = FileType {
    file_format: &FileFormat {
        id: 348_215_381,
        source_type: SourceType::Iana,
        name: "vnd.kde.kchart",
        extensions: &[],
        media_types: &["application/vnd.kde.kchart"],
        signatures: &[],
        related_formats: &[],
    },
};

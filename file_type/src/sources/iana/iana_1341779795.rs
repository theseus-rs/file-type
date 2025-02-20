use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1341779795: FileType = FileType {
    file_format: &FileFormat {
        id: 1_341_779_795,
        source_type: SourceType::Iana,
        name: "DV",
        extensions: &[],
        media_types: &["video/DV"],
        signatures: &[],
        related_formats: &[],
    },
};

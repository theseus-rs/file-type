use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2560879221: FileType = FileType {
    file_format: &FileFormat {
        id: 2_560_879_221,
        source_type: SourceType::Iana,
        name: "vnd.kde.kspread",
        extensions: &[],
        media_types: &["application/vnd.kde.kspread"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3076879620: FileType = FileType {
    file_format: &FileFormat {
        id: 3_076_879_620,
        source_type: SourceType::Iana,
        name: "vnd.kde.kword",
        extensions: &[],
        media_types: &["application/vnd.kde.kword"],
        signatures: &[],
        related_formats: &[],
    },
};

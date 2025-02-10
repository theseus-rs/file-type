use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3086374288: FileType = FileType {
    file_format: &FileFormat {
        id: 3_086_374_288,
        source_type: SourceType::Iana,
        name: "vnd.dece.graphic",
        extensions: &[],
        media_types: &["image/vnd.dece.graphic"],
        signatures: &[],
        related_formats: &[],
    },
};

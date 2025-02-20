use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3506818511: FileType = FileType {
    file_format: &FileFormat {
        id: 3_506_818_511,
        source_type: SourceType::Iana,
        name: "vnd.ms-playready.media.pya",
        extensions: &[],
        media_types: &["audio/vnd.ms-playready.media.pya"],
        signatures: &[],
        related_formats: &[],
    },
};

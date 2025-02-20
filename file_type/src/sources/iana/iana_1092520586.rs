use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1092520586: FileType = FileType {
    file_format: &FileFormat {
        id: 1_092_520_586,
        source_type: SourceType::Iana,
        name: "vnd.vocalshaper.vsp4",
        extensions: &[],
        media_types: &["application/vnd.vocalshaper.vsp4"],
        signatures: &[],
        related_formats: &[],
    },
};

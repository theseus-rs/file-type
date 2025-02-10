use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1914911530: FileType = FileType {
    file_format: &FileFormat {
        id: 1_914_911_530,
        source_type: SourceType::Iana,
        name: "iLBC",
        extensions: &[],
        media_types: &["audio/iLBC"],
        signatures: &[],
        related_formats: &[],
    },
};

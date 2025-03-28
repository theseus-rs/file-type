use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

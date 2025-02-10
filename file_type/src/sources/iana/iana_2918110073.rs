use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2918110073: FileType = FileType {
    file_format: &FileFormat {
        id: 2_918_110_073,
        source_type: SourceType::Iana,
        name: "bufr",
        extensions: &[],
        media_types: &["application/bufr"],
        signatures: &[],
        related_formats: &[],
    },
};

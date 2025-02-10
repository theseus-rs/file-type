use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3939241073: FileType = FileType {
    file_format: &FileFormat {
        id: 3_939_241_073,
        source_type: SourceType::Iana,
        name: "vnd.lotus-wordpro",
        extensions: &[],
        media_types: &["application/vnd.lotus-wordpro"],
        signatures: &[],
        related_formats: &[],
    },
};

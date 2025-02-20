use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

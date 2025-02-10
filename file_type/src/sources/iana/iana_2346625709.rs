use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2346625709: FileType = FileType {
    file_format: &FileFormat {
        id: 2_346_625_709,
        source_type: SourceType::Iana,
        name: "vnd.geonext",
        extensions: &[],
        media_types: &["application/vnd.geonext"],
        signatures: &[],
        related_formats: &[],
    },
};

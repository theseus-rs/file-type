use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2452209651: FileType = FileType {
    file_format: &FileFormat {
        id: 2_452_209_651,
        source_type: SourceType::Iana,
        name: "vnd.geospace",
        extensions: &[],
        media_types: &["application/vnd.geospace"],
        signatures: &[],
        related_formats: &[],
    },
};

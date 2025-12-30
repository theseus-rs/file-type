use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2168758570: FileType = FileType {
    file_format: &FileFormat {
        id: 2_168_758_570,
        source_type: SourceType::Iana,
        name: "vnd.as207960.vas.config+uper",
        extensions: &[],
        media_types: &["application/vnd.as207960.vas.config+uper"],
        signatures: &[],
        related_formats: &[],
    },
};

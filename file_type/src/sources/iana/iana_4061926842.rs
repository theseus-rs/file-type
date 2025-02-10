use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4061926842: FileType = FileType {
    file_format: &FileFormat {
        id: 4_061_926_842,
        source_type: SourceType::Iana,
        name: "vnd.dece.zip",
        extensions: &[],
        media_types: &["application/vnd.dece.zip"],
        signatures: &[],
        related_formats: &[],
    },
};

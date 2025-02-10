use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4207666045: FileType = FileType {
    file_format: &FileFormat {
        id: 4_207_666_045,
        source_type: SourceType::Iana,
        name: "vnd.las.las+json",
        extensions: &[],
        media_types: &["application/vnd.las.las+json"],
        signatures: &[],
        related_formats: &[],
    },
};

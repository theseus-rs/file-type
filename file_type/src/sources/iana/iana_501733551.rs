use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_501733551: FileType = FileType {
    file_format: &FileFormat {
        id: 501_733_551,
        source_type: SourceType::Iana,
        name: "vnd.svd",
        extensions: &[],
        media_types: &["application/vnd.svd"],
        signatures: &[],
        related_formats: &[],
    },
};

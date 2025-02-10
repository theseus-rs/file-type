use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_389201762: FileType = FileType {
    file_format: &FileFormat {
        id: 389_201_762,
        source_type: SourceType::Iana,
        name: "vnd.xfdl.webform",
        extensions: &[],
        media_types: &["application/vnd.xfdl.webform"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3546824522: FileType = FileType {
    file_format: &FileFormat {
        id: 3_546_824_522,
        source_type: SourceType::Iana,
        name: "vnd.groove-injector",
        extensions: &[],
        media_types: &["application/vnd.groove-injector"],
        signatures: &[],
        related_formats: &[],
    },
};

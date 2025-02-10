use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_734796685: FileType = FileType {
    file_format: &FileFormat {
        id: 734_796_685,
        source_type: SourceType::Iana,
        name: "jscontact+json",
        extensions: &[],
        media_types: &["application/jscontact+json"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3757341750: FileType = FileType {
    file_format: &FileFormat {
        id: 3_757_341_750,
        source_type: SourceType::Iana,
        name: "vnd.innopath.wamp.notification",
        extensions: &[],
        media_types: &["application/vnd.innopath.wamp.notification"],
        signatures: &[],
        related_formats: &[],
    },
};

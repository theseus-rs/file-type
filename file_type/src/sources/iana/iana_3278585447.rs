use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3278585447: FileType = FileType {
    file_format: &FileFormat {
        id: 3_278_585_447,
        source_type: SourceType::Iana,
        name: "vnd.groove-vcard",
        extensions: &[],
        media_types: &["application/vnd.groove-vcard"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3229630693: FileType = FileType {
    file_format: &FileFormat {
        id: 3_229_630_693,
        source_type: SourceType::Iana,
        name: "vnd.lotus-organizer",
        extensions: &[],
        media_types: &["application/vnd.lotus-organizer"],
        signatures: &[],
        related_formats: &[],
    },
};

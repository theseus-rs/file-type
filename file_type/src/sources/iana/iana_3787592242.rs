use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3787592242: FileType = FileType {
    file_format: &FileFormat {
        id: 3_787_592_242,
        source_type: SourceType::Iana,
        name: "vnd.onepagertamx",
        extensions: &[],
        media_types: &["application/vnd.onepagertamx"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2671377131: FileType = FileType {
    file_format: &FileFormat {
        id: 2_671_377_131,
        source_type: SourceType::Iana,
        name: "vnd.smart.teacher",
        extensions: &[],
        media_types: &["application/vnd.smart.teacher"],
        signatures: &[],
        related_formats: &[],
    },
};

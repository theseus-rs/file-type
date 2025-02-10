use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1593720423: FileType = FileType {
    file_format: &FileFormat {
        id: 1_593_720_423,
        source_type: SourceType::Iana,
        name: "vnd.koan",
        extensions: &[],
        media_types: &["application/vnd.koan"],
        signatures: &[],
        related_formats: &[],
    },
};

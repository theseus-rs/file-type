use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_664677117: FileType = FileType {
    file_format: &FileFormat {
        id: 664_677_117,
        source_type: SourceType::Iana,
        name: "vnd.nokia.radio-preset",
        extensions: &[],
        media_types: &["application/vnd.nokia.radio-preset"],
        signatures: &[],
        related_formats: &[],
    },
};

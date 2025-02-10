use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2814730858: FileType = FileType {
    file_format: &FileFormat {
        id: 2_814_730_858,
        source_type: SourceType::Iana,
        name: "jxrA",
        extensions: &[],
        media_types: &["image/jxrA"],
        signatures: &[],
        related_formats: &[],
    },
};

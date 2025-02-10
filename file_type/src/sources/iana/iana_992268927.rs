use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_992268927: FileType = FileType {
    file_format: &FileFormat {
        id: 992_268_927,
        source_type: SourceType::Iana,
        name: "ac3",
        extensions: &[],
        media_types: &["audio/ac3"],
        signatures: &[],
        related_formats: &[],
    },
};

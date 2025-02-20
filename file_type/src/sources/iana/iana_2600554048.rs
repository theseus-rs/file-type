use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2600554048: FileType = FileType {
    file_format: &FileFormat {
        id: 2_600_554_048,
        source_type: SourceType::Iana,
        name: "mikey",
        extensions: &[],
        media_types: &["application/mikey"],
        signatures: &[],
        related_formats: &[],
    },
};

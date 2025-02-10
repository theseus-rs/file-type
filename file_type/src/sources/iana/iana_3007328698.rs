use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3007328698: FileType = FileType {
    file_format: &FileFormat {
        id: 3_007_328_698,
        source_type: SourceType::Iana,
        name: "vnd.motorola.flexsuite.wem",
        extensions: &[],
        media_types: &["application/vnd.motorola.flexsuite.wem"],
        signatures: &[],
        related_formats: &[],
    },
};

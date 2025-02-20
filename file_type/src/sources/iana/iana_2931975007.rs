use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2931975007: FileType = FileType {
    file_format: &FileFormat {
        id: 2_931_975_007,
        source_type: SourceType::Iana,
        name: "vnd.motorola.flexsuite.ttc",
        extensions: &[],
        media_types: &["application/vnd.motorola.flexsuite.ttc"],
        signatures: &[],
        related_formats: &[],
    },
};

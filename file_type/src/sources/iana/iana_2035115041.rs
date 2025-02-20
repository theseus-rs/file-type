use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2035115041: FileType = FileType {
    file_format: &FileFormat {
        id: 2_035_115_041,
        source_type: SourceType::Iana,
        name: "cdmi-container",
        extensions: &[],
        media_types: &["application/cdmi-container"],
        signatures: &[],
        related_formats: &[],
    },
};

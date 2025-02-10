use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1610331905: FileType = FileType {
    file_format: &FileFormat {
        id: 1_610_331_905,
        source_type: SourceType::Iana,
        name: "vnd.valve.source.material",
        extensions: &[],
        media_types: &["application/vnd.valve.source.material"],
        signatures: &[],
        related_formats: &[],
    },
};

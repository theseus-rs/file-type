use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1331721474: FileType = FileType {
    file_format: &FileFormat {
        id: 1_331_721_474,
        source_type: SourceType::Iana,
        name: "dec-dx",
        extensions: &[],
        media_types: &["application/dec-dx"],
        signatures: &[],
        related_formats: &[],
    },
};

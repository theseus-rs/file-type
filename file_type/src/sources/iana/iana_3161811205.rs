use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3161811205: FileType = FileType {
    file_format: &FileFormat {
        id: 3_161_811_205,
        source_type: SourceType::Iana,
        name: "vnd.fafm+yaml",
        extensions: &[],
        media_types: &["application/vnd.fafm+yaml"],
        signatures: &[],
        related_formats: &[],
    },
};

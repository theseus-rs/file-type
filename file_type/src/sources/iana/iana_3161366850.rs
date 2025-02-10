use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3161366850: FileType = FileType {
    file_format: &FileFormat {
        id: 3_161_366_850,
        source_type: SourceType::Iana,
        name: "vnd.ms-lrm",
        extensions: &[],
        media_types: &["application/vnd.ms-lrm"],
        signatures: &[],
        related_formats: &[],
    },
};

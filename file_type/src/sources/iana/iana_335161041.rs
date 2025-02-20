use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_335161041: FileType = FileType {
    file_format: &FileFormat {
        id: 335_161_041,
        source_type: SourceType::Iana,
        name: "vnd.hydrostatix.sof-data",
        extensions: &[],
        media_types: &["application/vnd.hydrostatix.sof-data"],
        signatures: &[],
        related_formats: &[],
    },
};

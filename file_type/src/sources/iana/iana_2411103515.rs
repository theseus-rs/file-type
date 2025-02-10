use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2411103515: FileType = FileType {
    file_format: &FileFormat {
        id: 2_411_103_515,
        source_type: SourceType::Iana,
        name: "vnd.syft+json",
        extensions: &[],
        media_types: &["application/vnd.syft+json"],
        signatures: &[],
        related_formats: &[],
    },
};

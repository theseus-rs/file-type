use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2985677434: FileType = FileType {
    file_format: &FileFormat {
        id: 2_985_677_434,
        source_type: SourceType::Iana,
        name: "vnd.audiograph",
        extensions: &[],
        media_types: &["application/vnd.audiograph"],
        signatures: &[],
        related_formats: &[],
    },
};

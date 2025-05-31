use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4069413977: FileType = FileType {
    file_format: &FileFormat {
        id: 4_069_413_977,
        source_type: SourceType::Iana,
        name: "suit-envelope+cose",
        extensions: &[],
        media_types: &["application/suit-envelope+cose"],
        signatures: &[],
        related_formats: &[],
    },
};

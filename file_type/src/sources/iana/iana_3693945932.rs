use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3693945932: FileType = FileType {
    file_format: &FileFormat {
        id: 3_693_945_932,
        source_type: SourceType::Iana,
        name: "vnd.ipld.dag-json",
        extensions: &[],
        media_types: &["application/vnd.ipld.dag-json"],
        signatures: &[],
        related_formats: &[],
    },
};

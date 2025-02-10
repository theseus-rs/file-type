use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2429679128: FileType = FileType {
    file_format: &FileFormat {
        id: 2_429_679_128,
        source_type: SourceType::Iana,
        name: "vnd.mcd",
        extensions: &[],
        media_types: &["application/vnd.mcd"],
        signatures: &[],
        related_formats: &[],
    },
};

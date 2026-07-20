use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2346079200: FileType = FileType {
    file_format: &FileFormat {
        id: 2_346_079_200,
        source_type: SourceType::Iana,
        name: "cmcd",
        extensions: &[],
        media_types: &["application/cmcd"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2007977406: FileType = FileType {
    file_format: &FileFormat {
        id: 2_007_977_406,
        source_type: SourceType::Iana,
        name: "ibe-pp-data",
        extensions: &[],
        media_types: &["application/ibe-pp-data"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1105128667: FileType = FileType {
    file_format: &FileFormat {
        id: 1_105_128_667,
        source_type: SourceType::Iana,
        name: "vnd.usdz+zip",
        extensions: &[],
        media_types: &["model/vnd.usdz+zip"],
        signatures: &[],
        related_formats: &[],
    },
};

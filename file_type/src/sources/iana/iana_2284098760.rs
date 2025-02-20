use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2284098760: FileType = FileType {
    file_format: &FileFormat {
        id: 2_284_098_760,
        source_type: SourceType::Iana,
        name: "u3d",
        extensions: &[],
        media_types: &["model/u3d"],
        signatures: &[],
        related_formats: &[],
    },
};

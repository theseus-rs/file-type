use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3815169: FileType = FileType {
    file_format: &FileFormat {
        id: 3_815_169,
        source_type: SourceType::Wikidata,
        name: "VP9",
        extensions: &[],
        media_types: &["video/VP9"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_43664: FileType = FileType {
    file_format: &FileFormat {
        id: 43_664,
        source_type: SourceType::Wikidata,
        name: "High Efficiency Video Coding",
        extensions: &[],
        media_types: &["video/H265"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117814320: FileType = FileType {
    file_format: &FileFormat {
        id: 117_814_320,
        source_type: SourceType::Wikidata,
        name: "Working Model 3D Document",
        extensions: &["wm3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

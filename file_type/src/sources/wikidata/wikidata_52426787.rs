use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52426787: FileType = FileType {
    file_format: &FileFormat {
        id: 52_426_787,
        source_type: SourceType::Wikidata,
        name: "XYWrite Document, version III+",
        extensions: &["xyp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

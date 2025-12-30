use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136670548: FileType = FileType {
    file_format: &FileFormat {
        id: 136_670_548,
        source_type: SourceType::Wikidata,
        name: "Blue Iris Video",
        extensions: &["bvr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136160357: FileType = FileType {
    file_format: &FileFormat {
        id: 136_160_357,
        source_type: SourceType::Wikidata,
        name: "Practical Scriptwriter File",
        extensions: &["fsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

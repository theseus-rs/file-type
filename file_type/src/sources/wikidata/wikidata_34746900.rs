use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34746900: FileType = FileType {
    file_format: &FileFormat {
        id: 34_746_900,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Data Miner Project File",
        extensions: &["sdm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

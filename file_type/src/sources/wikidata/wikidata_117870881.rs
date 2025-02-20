use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117870881: FileType = FileType {
    file_format: &FileFormat {
        id: 117_870_881,
        source_type: SourceType::Wikidata,
        name: "TriGem SoftFax file",
        extensions: &["tri"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2375766: FileType = FileType {
    file_format: &FileFormat {
        id: 2_375_766,
        source_type: SourceType::Wikidata,
        name: "Synchronized Accessible Media Interchange",
        extensions: &["sami", "smi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

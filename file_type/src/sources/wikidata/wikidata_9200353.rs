use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_9200353: FileType = FileType {
    file_format: &FileFormat {
        id: 9_200_353,
        source_type: SourceType::Wikidata,
        name: "DigiBooster PRO v2.x / DigiBooster 3 module",
        extensions: &["dbm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110994642: FileType = FileType {
    file_format: &FileFormat {
        id: 110_994_642,
        source_type: SourceType::Wikidata,
        name: "SnapShot Project File",
        extensions: &["ssp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207539: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_539,
        source_type: SourceType::Wikidata,
        name: "Xerox EDMICS-MMR",
        extensions: &["ed", "mmr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

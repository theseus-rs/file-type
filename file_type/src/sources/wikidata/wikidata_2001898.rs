use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2001898: FileType = FileType {
    file_format: &FileFormat {
        id: 2_001_898,
        source_type: SourceType::Wikidata,
        name: "Notation Interchange File Format",
        extensions: &["nif"],
        media_types: &["application/vnd.music-niff"],
        signatures: &[],
        related_formats: &[],
    },
};

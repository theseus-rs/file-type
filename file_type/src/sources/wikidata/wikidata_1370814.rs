use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1370814: FileType = FileType {
    file_format: &FileFormat {
        id: 1_370_814,
        source_type: SourceType::Wikidata,
        name: "Amiga Module",
        extensions: &["mod"],
        media_types: &["application/soundapp", "audio/med", "audio/x-mod"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138747252: FileType = FileType {
    file_format: &FileFormat {
        id: 138_747_252,
        source_type: SourceType::Wikidata,
        name: "Fusion 360 Archival Board file",
        extensions: &["fbrd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

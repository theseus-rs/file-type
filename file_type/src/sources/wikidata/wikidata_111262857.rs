use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111262857: FileType = FileType {
    file_format: &FileFormat {
        id: 111_262_857,
        source_type: SourceType::Wikidata,
        name: "G.711 A-law european telephony format",
        extensions: &["alaw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

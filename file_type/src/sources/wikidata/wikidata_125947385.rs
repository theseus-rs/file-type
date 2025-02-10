use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125947385: FileType = FileType {
    file_format: &FileFormat {
        id: 125_947_385,
        source_type: SourceType::Wikidata,
        name: "Finale Notation File 2014+",
        extensions: &["musx"],
        media_types: &["application/vnd.makemusic.notation"],
        signatures: &[],
        related_formats: &[],
    },
};

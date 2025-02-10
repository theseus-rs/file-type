use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125938431: FileType = FileType {
    file_format: &FileFormat {
        id: 125_938_431,
        source_type: SourceType::Wikidata,
        name: "Enigma Binary File 2",
        extensions: &["mus"],
        media_types: &["application/vnd.makemusic.notation"],
        signatures: &[],
        related_formats: &[],
    },
};

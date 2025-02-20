use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58526504: FileType = FileType {
    file_format: &FileFormat {
        id: 58_526_504,
        source_type: SourceType::Wikidata,
        name: "Enigma Binary File 3+",
        extensions: &["mus"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127378050: FileType = FileType {
    file_format: &FileFormat {
        id: 127_378_050,
        source_type: SourceType::Wikidata,
        name: "Pyrex Source Code File",
        extensions: &["pyx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

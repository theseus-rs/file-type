use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127378389: FileType = FileType {
    file_format: &FileFormat {
        id: 127_378_389,
        source_type: SourceType::Wikidata,
        name: "Genie source code file",
        extensions: &["gs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

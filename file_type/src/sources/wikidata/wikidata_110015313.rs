use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110015313: FileType = FileType {
    file_format: &FileFormat {
        id: 110_015_313,
        source_type: SourceType::Wikidata,
        name: "Autorun Maestro Menu File",
        extensions: &["mnu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

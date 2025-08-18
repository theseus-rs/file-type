use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135527264: FileType = FileType {
    file_format: &FileFormat {
        id: 135_527_264,
        source_type: SourceType::Wikidata,
        name: "Exodus-II mesh file",
        extensions: &["e", "exo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

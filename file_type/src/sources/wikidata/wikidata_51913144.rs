use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51913144: FileType = FileType {
    file_format: &FileFormat {
        id: 51_913_144,
        source_type: SourceType::Wikidata,
        name: "Instalit Script",
        extensions: &["pvd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

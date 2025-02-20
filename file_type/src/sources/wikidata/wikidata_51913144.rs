use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127260495: FileType = FileType {
    file_format: &FileFormat {
        id: 127_260_495,
        source_type: SourceType::Wikidata,
        name: "Abaqus output database",
        extensions: &["odb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

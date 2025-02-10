use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127260826: FileType = FileType {
    file_format: &FileFormat {
        id: 127_260_826,
        source_type: SourceType::Wikidata,
        name: "Abaqus substructure file",
        extensions: &["sim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

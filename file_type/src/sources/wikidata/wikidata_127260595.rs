use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127260595: FileType = FileType {
    file_format: &FileFormat {
        id: 127_260_595,
        source_type: SourceType::Wikidata,
        name: "Abaqus/CAE model database",
        extensions: &["cae"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135711239: FileType = FileType {
    file_format: &FileFormat {
        id: 135_711_239,
        source_type: SourceType::Wikidata,
        name: "Kinetic description file",
        extensions: &["kpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

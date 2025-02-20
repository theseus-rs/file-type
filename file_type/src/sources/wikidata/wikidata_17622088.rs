use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_17622088: FileType = FileType {
    file_format: &FileFormat {
        id: 17_622_088,
        source_type: SourceType::Wikidata,
        name: "Speedo",
        extensions: &["spd"],
        media_types: &["application/x-font-speedo"],
        signatures: &[],
        related_formats: &[],
    },
};

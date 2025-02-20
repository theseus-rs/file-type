use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600495: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_495,
        source_type: SourceType::Wikidata,
        name: "Dia",
        extensions: &["dia"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

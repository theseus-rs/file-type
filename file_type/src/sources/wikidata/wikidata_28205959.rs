use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205959: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_959,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive Red Channel",
        extensions: &["imr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

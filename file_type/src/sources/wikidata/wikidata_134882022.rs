use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134882022: FileType = FileType {
    file_format: &FileFormat {
        id: 134_882_022,
        source_type: SourceType::Wikidata,
        name: "Soild Edge Document",
        extensions: &["dtf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100324136: FileType = FileType {
    file_format: &FileFormat {
        id: 100_324_136,
        source_type: SourceType::Wikidata,
        name: "Corel Print House/Print Office Document, version 3",
        extensions: &["cpd", "cph", "cpo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

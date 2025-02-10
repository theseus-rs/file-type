use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100324042: FileType = FileType {
    file_format: &FileFormat {
        id: 100_324_042,
        source_type: SourceType::Wikidata,
        name: "Corel Print House Document, version 1",
        extensions: &["cpd", "cph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

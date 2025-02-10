use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111316260: FileType = FileType {
    file_format: &FileFormat {
        id: 111_316_260,
        source_type: SourceType::Wikidata,
        name: "Sample Cell II Mac instrument",
        extensions: &["ini"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

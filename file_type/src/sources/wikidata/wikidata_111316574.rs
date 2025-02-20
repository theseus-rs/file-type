use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111316574: FileType = FileType {
    file_format: &FileFormat {
        id: 111_316_574,
        source_type: SourceType::Wikidata,
        name: "Sample Cell II PC instrument",
        extensions: &["ins"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

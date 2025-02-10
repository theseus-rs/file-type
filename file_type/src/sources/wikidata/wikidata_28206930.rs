use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206930: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_930,
        source_type: SourceType::Wikidata,
        name: "PGX",
        extensions: &["pgx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

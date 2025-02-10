use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122947391: FileType = FileType {
    file_format: &FileFormat {
        id: 122_947_391,
        source_type: SourceType::Wikidata,
        name: "Windows Enhanced Metafile, version 3.0",
        extensions: &["emf", "emz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

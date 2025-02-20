use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122947259: FileType = FileType {
    file_format: &FileFormat {
        id: 122_947_259,
        source_type: SourceType::Wikidata,
        name: "Windows Enhanced Metafile, version 2.0",
        extensions: &["emf", "emz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

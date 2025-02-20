use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108836959: FileType = FileType {
    file_format: &FileFormat {
        id: 108_836_959,
        source_type: SourceType::Wikidata,
        name: "Nero ISO CD Compilation File",
        extensions: &["nri"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

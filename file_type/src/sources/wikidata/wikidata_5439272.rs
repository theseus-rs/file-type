use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5439272: FileType = FileType {
    file_format: &FileFormat {
        id: 5_439_272,
        source_type: SourceType::Wikidata,
        name: "Font Definition Block",
        extensions: &["fdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

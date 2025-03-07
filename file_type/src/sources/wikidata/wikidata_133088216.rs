use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133088216: FileType = FileType {
    file_format: &FileFormat {
        id: 133_088_216,
        source_type: SourceType::Wikidata,
        name: "CD Architect Project File 5",
        extensions: &["cdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28755755: FileType = FileType {
    file_format: &FileFormat {
        id: 28_755_755,
        source_type: SourceType::Wikidata,
        name: "FDI",
        extensions: &["fdi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

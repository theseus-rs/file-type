use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118611163: FileType = FileType {
    file_format: &FileFormat {
        id: 118_611_163,
        source_type: SourceType::Wikidata,
        name: "Resource Template File",
        extensions: &["rct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

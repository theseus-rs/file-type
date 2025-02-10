use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113402233: FileType = FileType {
    file_format: &FileFormat {
        id: 113_402_233,
        source_type: SourceType::Wikidata,
        name: "ZBrush MatCap",
        extensions: &["zmt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

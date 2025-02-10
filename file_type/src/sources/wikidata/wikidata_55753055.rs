use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55753055: FileType = FileType {
    file_format: &FileFormat {
        id: 55_753_055,
        source_type: SourceType::Wikidata,
        name: "Redcode Metadata File",
        extensions: &["rmd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

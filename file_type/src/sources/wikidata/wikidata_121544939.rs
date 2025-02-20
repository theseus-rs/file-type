use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121544939: FileType = FileType {
    file_format: &FileFormat {
        id: 121_544_939,
        source_type: SourceType::Wikidata,
        name: "At Home 2011 Tax Return File",
        extensions: &["t11"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

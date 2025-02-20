use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29905291: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_291,
        source_type: SourceType::Wikidata,
        name: "Self-Extracting Archive",
        extensions: &["sfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

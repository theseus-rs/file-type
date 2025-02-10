use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979372: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_372,
        source_type: SourceType::Wikidata,
        name: "Kate",
        extensions: &["ogx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

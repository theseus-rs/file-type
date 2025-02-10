use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_17072901: FileType = FileType {
    file_format: &FileFormat {
        id: 17_072_901,
        source_type: SourceType::Wikidata,
        name: "Open Game Engine Exchange",
        extensions: &["ogex"],
        media_types: &["model/vnd.opengex"],
        signatures: &[],
        related_formats: &[],
    },
};

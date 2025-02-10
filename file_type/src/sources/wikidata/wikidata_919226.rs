use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_919226: FileType = FileType {
    file_format: &FileFormat {
        id: 919_226,
        source_type: SourceType::Wikidata,
        name: "MPEG-1 Audio Layer I",
        extensions: &["m1a", "mp1"],
        media_types: &["audio/MPA", "audio/mpeg"],
        signatures: &[],
        related_formats: &[],
    },
};

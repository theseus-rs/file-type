use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_726218: FileType = FileType {
    file_format: &FileFormat {
        id: 726_218,
        source_type: SourceType::Wikidata,
        name: "XML User Interface Language",
        extensions: &["xul"],
        media_types: &["application/vnd.mozilla.xul+xml"],
        signatures: &[],
        related_formats: &[],
    },
};

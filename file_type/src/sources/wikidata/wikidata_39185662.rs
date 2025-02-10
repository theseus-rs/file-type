use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_39185662: FileType = FileType {
    file_format: &FileFormat {
        id: 39_185_662,
        source_type: SourceType::Wikidata,
        name: "AHK script",
        extensions: &["ahk"],
        media_types: &["text/x-autohotkey"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959833: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_833,
        source_type: SourceType::Wikidata,
        name: "Cool Edit/Audition Multi Track Session file",
        extensions: &["ses"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125208012: FileType = FileType {
    file_format: &FileFormat {
        id: 125_208_012,
        source_type: SourceType::Wikidata,
        name: "TaskJuggler project",
        extensions: &["tjp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

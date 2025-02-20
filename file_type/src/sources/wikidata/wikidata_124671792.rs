use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124671792: FileType = FileType {
    file_format: &FileFormat {
        id: 124_671_792,
        source_type: SourceType::Wikidata,
        name: "Archive eXchange Format",
        extensions: &["axf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

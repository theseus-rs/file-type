use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21652057: FileType = FileType {
    file_format: &FileFormat {
        id: 21_652_057,
        source_type: SourceType::Wikidata,
        name: "Game Cache File",
        extensions: &["gcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

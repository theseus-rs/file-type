use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132527488: FileType = FileType {
    file_format: &FileFormat {
        id: 132_527_488,
        source_type: SourceType::Wikidata,
        name: "LEGO Mindstorms EV3 Text File",
        extensions: &["rtf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

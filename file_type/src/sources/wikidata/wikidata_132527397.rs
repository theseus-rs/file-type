use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132527397: FileType = FileType {
    file_format: &FileFormat {
        id: 132_527_397,
        source_type: SourceType::Wikidata,
        name: "LEGO Mindstorms EV3 Sound File",
        extensions: &["rsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

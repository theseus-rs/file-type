use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132527425: FileType = FileType {
    file_format: &FileFormat {
        id: 132_527_425,
        source_type: SourceType::Wikidata,
        name: "LEGO Mindstorms EV3 Graphics and Images file format",
        extensions: &["rgf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

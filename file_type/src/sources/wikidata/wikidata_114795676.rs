use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114795676: FileType = FileType {
    file_format: &FileFormat {
        id: 114_795_676,
        source_type: SourceType::Wikidata,
        name: "ReadCube Enhanced PDF",
        extensions: &["epdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

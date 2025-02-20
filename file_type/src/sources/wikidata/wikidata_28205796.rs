use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205796: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_796,
        source_type: SourceType::Wikidata,
        name: "Master of Orion saved game",
        extensions: &["gam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

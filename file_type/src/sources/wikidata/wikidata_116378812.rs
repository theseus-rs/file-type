use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116378812: FileType = FileType {
    file_format: &FileFormat {
        id: 116_378_812,
        source_type: SourceType::Wikidata,
        name: "Act! Database File",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

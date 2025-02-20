use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59608340: FileType = FileType {
    file_format: &FileFormat {
        id: 59_608_340,
        source_type: SourceType::Wikidata,
        name: "KryoFlux 2.2 format",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

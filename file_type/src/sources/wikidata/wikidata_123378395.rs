use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123378395: FileType = FileType {
    file_format: &FileFormat {
        id: 123_378_395,
        source_type: SourceType::Wikidata,
        name: "Radiosity Solution file",
        extensions: &["lwr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

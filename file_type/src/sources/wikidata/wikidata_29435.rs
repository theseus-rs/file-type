use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29435: FileType = FileType {
    file_format: &FileFormat {
        id: 29_435,
        source_type: SourceType::Wikidata,
        name: "Dolby TrueHD",
        extensions: &["thd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

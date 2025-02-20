use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100235620: FileType = FileType {
    file_format: &FileFormat {
        id: 100_235_620,
        source_type: SourceType::Wikidata,
        name: "FARO WorkSpace File",
        extensions: &["fws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

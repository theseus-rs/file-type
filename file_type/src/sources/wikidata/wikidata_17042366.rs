use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_17042366: FileType = FileType {
    file_format: &FileFormat {
        id: 17_042_366,
        source_type: SourceType::Wikidata,
        name: "id Software Music Format",
        extensions: &["imf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

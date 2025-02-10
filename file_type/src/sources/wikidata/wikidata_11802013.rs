use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_11802013: FileType = FileType {
    file_format: &FileFormat {
        id: 11_802_013,
        source_type: SourceType::Wikidata,
        name: "Sega Dreamcast Texture Package Format",
        extensions: &["pvm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

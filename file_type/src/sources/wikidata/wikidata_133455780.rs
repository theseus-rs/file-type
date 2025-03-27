use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133455780: FileType = FileType {
    file_format: &FileFormat {
        id: 133_455_780,
        source_type: SourceType::Wikidata,
        name: "Aseprite Format",
        extensions: &["ase", "aseprite"],
        media_types: &["image/x-aseprite"],
        signatures: &[],
        related_formats: &[],
    },
};

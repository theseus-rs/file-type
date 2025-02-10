use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119157250: FileType = FileType {
    file_format: &FileFormat {
        id: 119_157_250,
        source_type: SourceType::Wikidata,
        name: "Digital Image Publishing File",
        extensions: &["php"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

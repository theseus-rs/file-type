use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28777718: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_718,
        source_type: SourceType::Wikidata,
        name: "National Transmission Format",
        extensions: &["ntf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119097093: FileType = FileType {
    file_format: &FileFormat {
        id: 119_097_093,
        source_type: SourceType::Wikidata,
        name: "Fireworks PNG",
        extensions: &["png"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

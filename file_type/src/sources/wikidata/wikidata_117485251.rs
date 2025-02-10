use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117485251: FileType = FileType {
    file_format: &FileFormat {
        id: 117_485_251,
        source_type: SourceType::Wikidata,
        name: "Direct Stream Digital Interchange File Format",
        extensions: &["dff"],
        media_types: &["audio/x-dff"],
        signatures: &[],
        related_formats: &[],
    },
};

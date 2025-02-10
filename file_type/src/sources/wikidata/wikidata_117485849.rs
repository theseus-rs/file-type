use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117485849: FileType = FileType {
    file_format: &FileFormat {
        id: 117_485_849,
        source_type: SourceType::Wikidata,
        name: "Audacity Project File 1.x",
        extensions: &["aup"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

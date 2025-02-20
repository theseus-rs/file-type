use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117485947: FileType = FileType {
    file_format: &FileFormat {
        id: 117_485_947,
        source_type: SourceType::Wikidata,
        name: "Audacity Project File 2.x",
        extensions: &["aup"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

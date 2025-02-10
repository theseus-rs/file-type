use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117869071: FileType = FileType {
    file_format: &FileFormat {
        id: 117_869_071,
        source_type: SourceType::Wikidata,
        name: "Relisys file",
        extensions: &["tef"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

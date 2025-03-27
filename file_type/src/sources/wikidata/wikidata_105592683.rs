use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592683: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_683,
        source_type: SourceType::Wikidata,
        name: "TablEdit",
        extensions: &["tef"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

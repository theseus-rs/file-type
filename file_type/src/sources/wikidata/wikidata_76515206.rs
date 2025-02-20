use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_76515206: FileType = FileType {
    file_format: &FileFormat {
        id: 76_515_206,
        source_type: SourceType::Wikidata,
        name: "Lotus 123 SQZ! compressed Worksheet",
        extensions: &["wk!"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

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

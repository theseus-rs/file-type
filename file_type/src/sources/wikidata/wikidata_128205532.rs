use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128205532: FileType = FileType {
    file_format: &FileFormat {
        id: 128_205_532,
        source_type: SourceType::Wikidata,
        name: "UDO source code file",
        extensions: &["udo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

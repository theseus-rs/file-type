use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60478916: FileType = FileType {
    file_format: &FileFormat {
        id: 60_478_916,
        source_type: SourceType::Wikidata,
        name: "Qsplat Model",
        extensions: &["qs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130393916: FileType = FileType {
    file_format: &FileFormat {
        id: 130_393_916,
        source_type: SourceType::Wikidata,
        name: "Actual Drawing file",
        extensions: &["adf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

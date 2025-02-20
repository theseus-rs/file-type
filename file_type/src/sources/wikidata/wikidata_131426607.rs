use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131426607: FileType = FileType {
    file_format: &FileFormat {
        id: 131_426_607,
        source_type: SourceType::Wikidata,
        name: "Wren source code file format",
        extensions: &["wren"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

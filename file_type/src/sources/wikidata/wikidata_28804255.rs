use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28804255: FileType = FileType {
    file_format: &FileFormat {
        id: 28_804_255,
        source_type: SourceType::Wikidata,
        name: "Uniform Office Spreadsheet",
        extensions: &["uos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

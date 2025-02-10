use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59713856: FileType = FileType {
    file_format: &FileFormat {
        id: 59_713_856,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Spreadsheet",
        extensions: &["xlr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

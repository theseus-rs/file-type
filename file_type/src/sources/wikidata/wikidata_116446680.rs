use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116446680: FileType = FileType {
    file_format: &FileFormat {
        id: 116_446_680,
        source_type: SourceType::Wikidata,
        name: "Microsoft Works Spreadsheet",
        extensions: &["wks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

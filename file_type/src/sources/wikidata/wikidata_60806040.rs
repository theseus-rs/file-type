use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60806040: FileType = FileType {
    file_format: &FileFormat {
        id: 60_806_040,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel 3.0 Worksheet (xls)",
        extensions: &["xls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

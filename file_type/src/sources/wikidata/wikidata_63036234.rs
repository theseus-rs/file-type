use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_63036234: FileType = FileType {
    file_format: &FileFormat {
        id: 63_036_234,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel 4.0 Workbook",
        extensions: &["xlw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

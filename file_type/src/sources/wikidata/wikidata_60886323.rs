use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60886323: FileType = FileType {
    file_format: &FileFormat {
        id: 60_886_323,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel 2000-2003 Workbook",
        extensions: &["xls", "xlw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

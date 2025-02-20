use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60886160: FileType = FileType {
    file_format: &FileFormat {
        id: 60_886_160,
        source_type: SourceType::Wikidata,
        name: "Microsoft Excel 97 Workbook",
        extensions: &["xls", "xlw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

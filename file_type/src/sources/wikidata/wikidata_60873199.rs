use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60873199: FileType = FileType {
    file_format: &FileFormat {
        id: 60_873_199,
        source_type: SourceType::Wikidata,
        name: "Excel 95 Workbook",
        extensions: &["xls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

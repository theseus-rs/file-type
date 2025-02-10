use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66759528: FileType = FileType {
    file_format: &FileFormat {
        id: 66_759_528,
        source_type: SourceType::Wikidata,
        name: "Excel Binary Workbook",
        extensions: &["xlsb"],
        media_types: &["application/vnd.ms-excel.sheet.binary.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27203692: FileType = FileType {
    file_format: &FileFormat {
        id: 27_203_692,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Spreadsheet, version 1.0",
        extensions: &["ods"],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
        signatures: &[],
        related_formats: &[],
    },
};

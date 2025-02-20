use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27203789: FileType = FileType {
    file_format: &FileFormat {
        id: 27_203_789,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Spreadsheet, version 1.2",
        extensions: &["ods"],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114061472: FileType = FileType {
    file_format: &FileFormat {
        id: 114_061_472,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Spreadsheet, version 1.3",
        extensions: &["ods"],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
        signatures: &[],
        related_formats: &[],
    },
};

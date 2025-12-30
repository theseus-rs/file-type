use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136405659: FileType = FileType {
    file_format: &FileFormat {
        id: 136_405_659,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Spreadsheet, version 1.4",
        extensions: &["ods"],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet"],
        signatures: &[],
        related_formats: &[],
    },
};

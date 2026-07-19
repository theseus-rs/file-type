use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_140370270: FileType = FileType {
    file_format: &FileFormat {
        id: 140_370_270,
        source_type: SourceType::Wikidata,
        name: "OpenOffice.org 1.0 Spreadsheet Document",
        extensions: &["sxc"],
        media_types: &["application/vnd.sun.xml.calc"],
        signatures: &[],
        related_formats: &[],
    },
};

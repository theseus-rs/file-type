use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111673769: FileType = FileType {
    file_format: &FileFormat {
        id: 111_673_769,
        source_type: SourceType::Wikidata,
        name: "Kingsoft Spreadsheets Template",
        extensions: &["ett"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111648762: FileType = FileType {
    file_format: &FileFormat {
        id: 111_648_762,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Scrapbook Page File",
        extensions: &["sbp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

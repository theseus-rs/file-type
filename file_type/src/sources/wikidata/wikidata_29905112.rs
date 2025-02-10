use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29905112: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_112,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System item store file",
        extensions: &["sas7bitm", "sr7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

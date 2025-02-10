use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_115606990: FileType = FileType {
    file_format: &FileFormat {
        id: 115_606_990,
        source_type: SourceType::Wikidata,
        name: "VCD Layout File",
        extensions: &["vcl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

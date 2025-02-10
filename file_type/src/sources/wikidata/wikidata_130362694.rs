use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130362694: FileType = FileType {
    file_format: &FileFormat {
        id: 130_362_694,
        source_type: SourceType::Wikidata,
        name: "Myghty file format",
        extensions: &["myt"],
        media_types: &["application/x-myghty"],
        signatures: &[],
        related_formats: &[],
    },
};

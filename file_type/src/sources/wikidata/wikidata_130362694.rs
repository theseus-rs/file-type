use crate::FileType;
use crate::format::{FileFormat, SourceType};

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

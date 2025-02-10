use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_901031: FileType = FileType {
    file_format: &FileFormat {
        id: 901_031,
        source_type: SourceType::Wikidata,
        name: "device independent file format",
        extensions: &["dvi"],
        media_types: &["application/x-dvi"],
        signatures: &[],
        related_formats: &[],
    },
};

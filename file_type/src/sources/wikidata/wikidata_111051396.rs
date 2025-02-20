use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111051396: FileType = FileType {
    file_format: &FileFormat {
        id: 111_051_396,
        source_type: SourceType::Wikidata,
        name: "WebAssembly text format",
        extensions: &["wast", "wat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

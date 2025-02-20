use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47487577: FileType = FileType {
    file_format: &FileFormat {
        id: 47_487_577,
        source_type: SourceType::Wikidata,
        name: "Alias Scene Description Language",
        extensions: &["sdl"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};

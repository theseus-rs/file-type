use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206359: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_359,
        source_type: SourceType::Wikidata,
        name: "Intergraph Raster",
        extensions: &[
            "cit", "cmp", "cot", "crl", "ctb", "ctc", "g3", "g4", "grd", "ing", "itg", "jpg",
            "lsr", "rgb", "rle", "t27", "t29", "tg4", "tpe",
        ],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};

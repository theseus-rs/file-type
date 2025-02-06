use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206359: FileFormat = FileFormat {
    id: 28_206_359,
    source_type: SourceType::Wikidata,
    name: "Intergraph Raster",
    extensions: &[
        "cit", "cmp", "cot", "crl", "ctb", "ctc", "g3", "g4", "grd", "ing", "itg", "jpg", "lsr",
        "rgb", "rle", "t27", "t29", "tg4", "tpe",
    ],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

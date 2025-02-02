use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206359: FileFormat = FileFormat {
    id: 28_206_359,
    source_type: SourceType::Wikidata,
    name: "Intergraph Raster",
    extensions: &[
        "cit", "cmp", "cot", "crl", "ctb", "ctc", "g3", "g4", "grd", "ing", "itg", "jpg", "lsr",
        "rgb", "rle", "t27", "t29", "tg4", "tpe",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117287127: FileFormat = FileFormat {
    id: 117_287_127,
    puid: "wikidata/117287127",
    name: "SigmaPlot 1.0, 2.0 Worksheet",
    extensions: &["spw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130372599: FileFormat = FileFormat {
    id: 130_372_599,
    puid: "wikidata/130372599",
    name: "NestedText file format",
    extensions: &["nt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

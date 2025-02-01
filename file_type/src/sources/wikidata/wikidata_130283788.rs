use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130283788: FileFormat = FileFormat {
    id: 130_283_788,
    puid: "wikidata/130283788",
    name: "Maxima file format",
    extensions: &["mac", "max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

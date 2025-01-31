use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130726128: FileFormat = FileFormat {
    id: 130_726_128,
    puid: "wikidata/130726128",
    name: "S source code file",
    extensions: &["S"],
    media_types: &["text/S"],
    internal_signatures: &[],
    related_formats: &[],
};

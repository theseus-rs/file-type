use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919166: FileFormat = FileFormat {
    id: 28_919_166,
    puid: "wikidata/28919166",
    name: "GHS Geometry",
    extensions: &["gf", "gft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

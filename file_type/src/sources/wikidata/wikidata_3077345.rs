use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3077345: FileFormat = FileFormat {
    id: 3_077_345,
    puid: "wikidata/3077345",
    name: "Polygon File Format",
    extensions: &["ply"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};

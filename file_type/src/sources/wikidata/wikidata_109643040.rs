use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109643040: FileFormat = FileFormat {
    id: 109_643_040,
    puid: "wikidata/109643040",
    name: "VJ file format",
    extensions: &["vj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

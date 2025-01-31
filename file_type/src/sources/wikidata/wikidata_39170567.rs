use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_39170567: FileFormat = FileFormat {
    id: 39_170_567,
    puid: "wikidata/39170567",
    name: "GeoGebra tool",
    extensions: &["ggt"],
    media_types: &["application/vnd.geogebra.tool"],
    internal_signatures: &[],
    related_formats: &[],
};

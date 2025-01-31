use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75597003: FileFormat = FileFormat {
    id: 75_597_003,
    puid: "wikidata/75597003",
    name: "GeoGebra format, version 1.0",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[],
};

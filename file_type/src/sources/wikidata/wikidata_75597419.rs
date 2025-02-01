use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75597419: FileFormat = FileFormat {
    id: 75_597_419,
    puid: "wikidata/75597419",
    name: "GeoGebra format, version 2.0",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[],
};

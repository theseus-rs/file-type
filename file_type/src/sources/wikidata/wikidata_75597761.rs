use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_75597761: FileFormat = FileFormat {
    id: 75_597_761,
    puid: "wikidata/75597761",
    name: "GeoGebra format, version 1.x",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[],
};

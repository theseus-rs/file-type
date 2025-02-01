use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28921959: FileFormat = FileFormat {
    id: 28_921_959,
    puid: "wikidata/28921959",
    name: "Keyhole Markup Language Zipped",
    extensions: &["kmz"],
    media_types: &["application/vnd.google-earth.kmz"],
    internal_signatures: &[],
    related_formats: &[],
};

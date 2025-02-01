use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110456179: FileFormat = FileFormat {
    id: 110_456_179,
    puid: "wikidata/110456179",
    name: "Standard Data Format",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

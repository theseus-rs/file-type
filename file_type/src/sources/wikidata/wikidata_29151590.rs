use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29151590: FileFormat = FileFormat {
    id: 29_151_590,
    puid: "wikidata/29151590",
    name: "Redcode oBJect",
    extensions: &["rbj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124845089: FileFormat = FileFormat {
    id: 124_845_089,
    puid: "wikidata/124845089",
    name: "mh",
    extensions: &["mh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

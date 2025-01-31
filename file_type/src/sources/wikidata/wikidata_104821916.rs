use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_104821916: FileFormat = FileFormat {
    id: 104_821_916,
    puid: "wikidata/104821916",
    name: "Renoise instrument",
    extensions: &["rni", "xrni"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

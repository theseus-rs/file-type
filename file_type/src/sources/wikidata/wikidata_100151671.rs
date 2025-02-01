use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100151671: FileFormat = FileFormat {
    id: 100_151_671,
    puid: "wikidata/100151671",
    name: "Bruker PDZ",
    extensions: &["pdz", "xpdz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

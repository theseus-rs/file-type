use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967223: FileFormat = FileFormat {
    id: 27_967_223,
    puid: "wikidata/27967223",
    name: "StarTrekker/Star Tracker module",
    extensions: &["mod", "nt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

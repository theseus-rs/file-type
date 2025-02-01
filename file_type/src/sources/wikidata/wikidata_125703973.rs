use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125703973: FileFormat = FileFormat {
    id: 125_703_973,
    puid: "wikidata/125703973",
    name: "StarWriter 4.0/5.0 Master Document",
    extensions: &["sgl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

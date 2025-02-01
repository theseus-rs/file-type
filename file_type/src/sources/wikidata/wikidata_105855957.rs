use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855957: FileFormat = FileFormat {
    id: 105_855_957,
    puid: "wikidata/105855957",
    name: "ColdFusion Verity engine fields definition",
    extensions: &["ddd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

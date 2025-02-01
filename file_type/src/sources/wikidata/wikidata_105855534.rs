use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855534: FileFormat = FileFormat {
    id: 105_855_534,
    puid: "wikidata/105855534",
    name: "Open Digital Rights Language",
    extensions: &["dr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

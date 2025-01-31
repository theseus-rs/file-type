use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110135368: FileFormat = FileFormat {
    id: 110_135_368,
    puid: "wikidata/110135368",
    name: "Serif PhotoPlus Image, version 5-X2",
    extensions: &["spp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

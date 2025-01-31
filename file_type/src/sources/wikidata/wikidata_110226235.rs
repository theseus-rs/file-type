use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110226235: FileFormat = FileFormat {
    id: 110_226_235,
    puid: "wikidata/110226235",
    name: "NeoDesk Icon File, version 3",
    extensions: &["nic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

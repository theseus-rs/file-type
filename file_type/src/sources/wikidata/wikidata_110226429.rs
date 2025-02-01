use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110226429: FileFormat = FileFormat {
    id: 110_226_429,
    puid: "wikidata/110226429",
    name: "ELAN Preference File",
    extensions: &["pfsx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

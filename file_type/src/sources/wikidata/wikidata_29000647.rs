use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000647: FileFormat = FileFormat {
    id: 29_000_647,
    puid: "wikidata/29000647",
    name: "PLG",
    extensions: &["plg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

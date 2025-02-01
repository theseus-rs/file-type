use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128034054: FileFormat = FileFormat {
    id: 128_034_054,
    puid: "wikidata/128034054",
    name: "Rebol script",
    extensions: &["r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

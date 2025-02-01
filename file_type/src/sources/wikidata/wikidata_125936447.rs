use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125936447: FileFormat = FileFormat {
    id: 125_936_447,
    puid: "wikidata/125936447",
    name: "Atrac Codec File v.1",
    extensions: &["aea"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

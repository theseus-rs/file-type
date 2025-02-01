use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167429: FileFormat = FileFormat {
    id: 29_167_429,
    puid: "wikidata/29167429",
    name: "NovaMind",
    extensions: &["nmind"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

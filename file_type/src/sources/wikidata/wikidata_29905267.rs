use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905267: FileFormat = FileFormat {
    id: 29_905_267,
    puid: "wikidata/29905267",
    name: "Self-Extracting Archive",
    extensions: &["sea"],
    media_types: &["application/x-sea"],
    internal_signatures: &[],
    related_formats: &[],
};

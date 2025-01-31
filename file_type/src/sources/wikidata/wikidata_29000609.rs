use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29000609: FileFormat = FileFormat {
    id: 29_000_609,
    puid: "wikidata/29000609",
    name: "Java Card CAP",
    extensions: &["cap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

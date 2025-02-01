use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650340: FileFormat = FileFormat {
    id: 29_650_340,
    puid: "wikidata/29650340",
    name: "PES",
    extensions: &["pes"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

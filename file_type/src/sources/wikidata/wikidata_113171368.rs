use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113171368: FileFormat = FileFormat {
    id: 113_171_368,
    puid: "wikidata/113171368",
    name: "Family Lawyer Document",
    extensions: &["pfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

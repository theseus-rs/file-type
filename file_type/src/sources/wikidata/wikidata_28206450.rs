use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206450: FileFormat = FileFormat {
    id: 28_206_450,
    puid: "wikidata/28206450",
    name: "KiSS CEL 8-bit",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

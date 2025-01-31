use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206446: FileFormat = FileFormat {
    id: 28_206_446,
    puid: "wikidata/28206446",
    name: "KiSS CEL 4-bit",
    extensions: &["cel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61990487: FileFormat = FileFormat {
    id: 61_990_487,
    puid: "wikidata/61990487",
    name: "Log ASCII Standard Format, version 3",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

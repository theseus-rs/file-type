use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61990483: FileFormat = FileFormat {
    id: 61_990_483,
    puid: "wikidata/61990483",
    name: "Log ASCII Standard Format, version 2",
    extensions: &["las"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

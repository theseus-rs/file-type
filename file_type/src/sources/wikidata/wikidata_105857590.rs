use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857590: FileFormat = FileFormat {
    id: 105_857_590,
    puid: "wikidata/105857590",
    name: "ipuz puzzle open format",
    extensions: &["ipuz"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};

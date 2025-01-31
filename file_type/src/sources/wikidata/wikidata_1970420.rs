use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1970420: FileFormat = FileFormat {
    id: 1_970_420,
    puid: "wikidata/1970420",
    name: "Simple file verification",
    extensions: &["sfv"],
    media_types: &["text/x-sfv"],
    internal_signatures: &[],
    related_formats: &[],
};

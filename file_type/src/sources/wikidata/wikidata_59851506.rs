use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59851506: FileFormat = FileFormat {
    id: 59_851_506,
    puid: "wikidata/59851506",
    name: "DROID File Collection File Format",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

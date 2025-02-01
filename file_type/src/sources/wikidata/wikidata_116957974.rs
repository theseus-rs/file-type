use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116957974: FileFormat = FileFormat {
    id: 116_957_974,
    puid: "wikidata/116957974",
    name: "ICN AT&T/Multigen",
    extensions: &["icn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

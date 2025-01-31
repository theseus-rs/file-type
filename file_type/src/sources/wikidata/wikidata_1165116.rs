use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1165116: FileFormat = FileFormat {
    id: 1_165_116,
    puid: "wikidata/1165116",
    name: "Perl module",
    extensions: &["pm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

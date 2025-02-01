use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854292: FileFormat = FileFormat {
    id: 105_854_292,
    puid: "wikidata/105854292",
    name: "Quest Adventure Script (v5)",
    extensions: &["aslx"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

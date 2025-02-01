use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854934: FileFormat = FileFormat {
    id: 105_854_934,
    puid: "wikidata/105854934",
    name: "Abstract Markup Language",
    extensions: &["aml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

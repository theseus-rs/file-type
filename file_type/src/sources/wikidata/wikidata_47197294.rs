use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47197294: FileFormat = FileFormat {
    id: 47_197_294,
    puid: "wikidata/47197294",
    name: "AppleWorks Word Processor file format version 6",
    extensions: &["cwk", "cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

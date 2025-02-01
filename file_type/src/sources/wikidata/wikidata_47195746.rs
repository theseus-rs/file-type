use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47195746: FileFormat = FileFormat {
    id: 47_195_746,
    puid: "wikidata/47195746",
    name: "AppleWorks Word Processor file format, version 5",
    extensions: &["cwk", "cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

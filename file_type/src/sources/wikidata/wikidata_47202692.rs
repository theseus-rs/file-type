use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47202692: FileFormat = FileFormat {
    id: 47_202_692,
    puid: "wikidata/47202692",
    name: "AppleWorks Database file format version 6",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

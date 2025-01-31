use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47197074: FileFormat = FileFormat {
    id: 47_197_074,
    puid: "wikidata/47197074",
    name: "AppleWorks Drawing file format version 6",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

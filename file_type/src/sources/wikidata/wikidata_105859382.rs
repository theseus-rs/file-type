use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859382: FileFormat = FileFormat {
    id: 105_859_382,
    puid: "wikidata/105859382",
    name: "QTI document",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

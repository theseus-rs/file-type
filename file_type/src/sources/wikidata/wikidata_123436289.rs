use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123436289: FileFormat = FileFormat {
    id: 123_436_289,
    puid: "wikidata/123436289",
    name: "DARC-F1 file",
    extensions: &["f1d"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

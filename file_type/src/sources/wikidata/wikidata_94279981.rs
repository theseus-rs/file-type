use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_94279981: FileFormat = FileFormat {
    id: 94_279_981,
    puid: "wikidata/94279981",
    name: "Dragon",
    extensions: &["dgn"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

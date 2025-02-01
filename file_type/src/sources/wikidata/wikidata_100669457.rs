use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100669457: FileFormat = FileFormat {
    id: 100_669_457,
    puid: "wikidata/100669457",
    name: "Apple iWork Document, version 14",
    extensions: &["iwa", "key", "numbers", "pages", "template"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

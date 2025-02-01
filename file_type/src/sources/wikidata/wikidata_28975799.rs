use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975799: FileFormat = FileFormat {
    id: 28_975_799,
    puid: "wikidata/28975799",
    name: "FACT",
    extensions: &["fact"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

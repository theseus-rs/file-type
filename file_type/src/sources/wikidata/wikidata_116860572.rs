use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116860572: FileFormat = FileFormat {
    id: 116_860_572,
    puid: "wikidata/116860572",
    name: "Stock Tracker Report File",
    extensions: &["srw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

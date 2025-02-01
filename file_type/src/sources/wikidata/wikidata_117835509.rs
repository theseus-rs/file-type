use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117835509: FileFormat = FileFormat {
    id: 117_835_509,
    puid: "wikidata/117835509",
    name: "Generic fax format",
    extensions: &["cg3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

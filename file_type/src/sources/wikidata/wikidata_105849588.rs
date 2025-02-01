use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849588: FileFormat = FileFormat {
    id: 105_849_588,
    puid: "wikidata/105849588",
    name: "Ventura Publisher Caption",
    extensions: &["cap"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

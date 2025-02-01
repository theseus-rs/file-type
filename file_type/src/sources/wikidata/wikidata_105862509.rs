use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862509: FileFormat = FileFormat {
    id: 105_862_509,
    puid: "wikidata/105862509",
    name: "Max Patch",
    extensions: &["maxpat"],
    media_types: &["text/json"],
    internal_signatures: &[],
    related_formats: &[],
};

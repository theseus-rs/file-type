use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849272: FileFormat = FileFormat {
    id: 105_849_272,
    puid: "wikidata/105849272",
    name: "GNU Bison grammar",
    extensions: &["yy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

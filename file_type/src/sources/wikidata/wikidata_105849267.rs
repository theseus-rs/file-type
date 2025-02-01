use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849267: FileFormat = FileFormat {
    id: 105_849_267,
    puid: "wikidata/105849267",
    name: "GNU Bison grammar (with rem)",
    extensions: &["yy"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

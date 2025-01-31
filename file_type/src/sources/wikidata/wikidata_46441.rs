use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_46441: FileFormat = FileFormat {
    id: 46_441,
    puid: "wikidata/46441",
    name: "Cascading Style Sheets",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};

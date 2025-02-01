use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34305098: FileFormat = FileFormat {
    id: 34_305_098,
    puid: "wikidata/34305098",
    name: "Sassy Cascading Style Sheets",
    extensions: &["scss"],
    media_types: &["text/x-scss"],
    internal_signatures: &[],
    related_formats: &[],
};

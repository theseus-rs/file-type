use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34308772: FileFormat = FileFormat {
    id: 34_308_772,
    puid: "wikidata/34308772",
    name: "Scrivener document",
    extensions: &["scriv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113487065: FileFormat = FileFormat {
    id: 113_487_065,
    puid: "wikidata/113487065",
    name: "Persuasion Windows Document 2.0",
    extensions: &["at2", "pr2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

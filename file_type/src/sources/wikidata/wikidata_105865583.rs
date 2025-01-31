use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865583: FileFormat = FileFormat {
    id: 105_865_583,
    puid: "wikidata/105865583",
    name: "ProPresenter 4/5 presentation",
    extensions: &["pro4", "pro5"],
    media_types: &["text/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865035: FileFormat = FileFormat {
    id: 105_865_035,
    puid: "wikidata/105865035",
    name: "Eclipse Preferences (with rem)",
    extensions: &["prefs"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

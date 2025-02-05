use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865035: FileFormat = FileFormat {
    id: 105_865_035,
    source_type: SourceType::Wikidata,
    name: "Eclipse Preferences (with rem)",
    extensions: &["prefs"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};

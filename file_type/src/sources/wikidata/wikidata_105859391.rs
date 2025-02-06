use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859391: FileFormat = FileFormat {
    id: 105_859_391,
    source_type: SourceType::Wikidata,
    name: "SlickRun MagicWord Pack",
    extensions: &["qrs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

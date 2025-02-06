use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_5354833: FileFormat = FileFormat {
    id: 5_354_833,
    source_type: SourceType::Wikidata,
    name: "Qualcomm code-excited linear prediction",
    extensions: &["qcp"],
    media_types: &["audio/qcelp"],
    signatures: &[],
    related_formats: &[],
};

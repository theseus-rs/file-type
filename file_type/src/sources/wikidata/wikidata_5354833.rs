use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5354833: FileFormat = FileFormat {
    id: 5_354_833,
    puid: "wikidata/5354833",
    name: "Qualcomm code-excited linear prediction",
    extensions: &["qcp"],
    media_types: &["audio/qcelp"],
    internal_signatures: &[],
    related_formats: &[],
};

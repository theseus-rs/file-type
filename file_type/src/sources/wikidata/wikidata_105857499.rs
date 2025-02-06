use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857499: FileFormat = FileFormat {
    id: 105_857_499,
    source_type: SourceType::Wikidata,
    name: "Shaper LUT (with rem)",
    extensions: &["3dl"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};

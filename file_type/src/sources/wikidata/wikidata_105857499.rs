use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857499: FileFormat = FileFormat {
    id: 105_857_499,
    source_type: SourceType::Wikidata,
    name: "Shaper LUT (with rem)",
    extensions: &["3dl"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};

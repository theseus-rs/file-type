use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854356: FileFormat = FileFormat {
    id: 105_854_356,
    source_type: SourceType::Wikidata,
    name: "X-Plane Airfoils",
    extensions: &["afl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

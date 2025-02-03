use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854356: FileFormat = FileFormat {
    id: 105_854_356,
    source_type: SourceType::Wikidata,
    name: "X-Plane Airfoils",
    extensions: &["afl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

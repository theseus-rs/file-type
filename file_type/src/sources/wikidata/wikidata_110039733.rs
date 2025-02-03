use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110039733: FileFormat = FileFormat {
    id: 110_039_733,
    source_type: SourceType::Wikidata,
    name: "Mar Archive",
    extensions: &["mac", "mar"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

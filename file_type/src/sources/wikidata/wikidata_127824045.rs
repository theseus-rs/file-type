use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127824045: FileFormat = FileFormat {
    id: 127_824_045,
    source_type: SourceType::Wikidata,
    name: "Cinema DTS Audio file format",
    extensions: &["apx", "aud", "aue"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

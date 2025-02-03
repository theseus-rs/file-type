use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100301097: FileFormat = FileFormat {
    id: 100_301_097,
    source_type: SourceType::Wikidata,
    name: "Flow Charting PDQ format",
    extensions: &["pdq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

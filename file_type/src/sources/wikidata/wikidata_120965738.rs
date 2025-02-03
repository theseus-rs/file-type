use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120965738: FileFormat = FileFormat {
    id: 120_965_738,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 95 data file",
    extensions: &["mn4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

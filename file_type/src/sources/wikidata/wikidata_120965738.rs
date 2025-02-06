use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120965738: FileFormat = FileFormat {
    id: 120_965_738,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 95 data file",
    extensions: &["mn4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

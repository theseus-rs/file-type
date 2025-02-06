use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120920869: FileFormat = FileFormat {
    id: 120_920_869,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2007 data file",
    extensions: &["m16"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

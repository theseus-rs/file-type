use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120920869: FileFormat = FileFormat {
    id: 120_920_869,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2007 data file",
    extensions: &["m16"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

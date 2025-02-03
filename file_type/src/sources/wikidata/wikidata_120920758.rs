use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120920758: FileFormat = FileFormat {
    id: 120_920_758,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2005 backup data",
    extensions: &["m14"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

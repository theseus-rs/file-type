use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120920266: FileFormat = FileFormat {
    id: 120_920_266,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money version 1 data",
    extensions: &["mn1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

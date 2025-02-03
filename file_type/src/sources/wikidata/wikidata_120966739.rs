use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120966739: FileFormat = FileFormat {
    id: 120_966_739,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2001 data",
    extensions: &["mn9"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

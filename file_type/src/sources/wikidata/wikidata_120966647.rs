use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120966647: FileFormat = FileFormat {
    id: 120_966_647,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money 2000 data file",
    extensions: &["mn8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

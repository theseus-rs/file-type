use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120965378: FileFormat = FileFormat {
    id: 120_965_378,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money version 2 data",
    extensions: &["mn2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

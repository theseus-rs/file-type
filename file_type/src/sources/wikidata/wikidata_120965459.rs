use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120965459: FileFormat = FileFormat {
    id: 120_965_459,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money version 3 data",
    extensions: &["mn3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_268086: FileFormat = FileFormat {
    id: 268_086,
    source_type: SourceType::Wikidata,
    name: "OMDoc",
    extensions: &["omdoc"],
    media_types: &["application/omdoc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};

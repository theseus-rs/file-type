use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117457865: FileFormat = FileFormat {
    id: 117_457_865,
    source_type: SourceType::Wikidata,
    name: "XDOMEA 3.0.0",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206579: FileFormat = FileFormat {
    id: 28_206_579,
    source_type: SourceType::Wikidata,
    name: "MetaMorph Stack",
    extensions: &["stk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

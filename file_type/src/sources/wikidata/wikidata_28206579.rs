use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206579: FileFormat = FileFormat {
    id: 28_206_579,
    source_type: SourceType::Wikidata,
    name: "MetaMorph Stack",
    extensions: &["stk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

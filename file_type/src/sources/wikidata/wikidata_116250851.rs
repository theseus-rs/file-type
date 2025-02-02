use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116250851: FileFormat = FileFormat {
    id: 116_250_851,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior Project file",
    extensions: &["mcp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};

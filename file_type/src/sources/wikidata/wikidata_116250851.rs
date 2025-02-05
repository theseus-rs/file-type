use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116250851: FileFormat = FileFormat {
    id: 116_250_851,
    source_type: SourceType::Wikidata,
    name: "CodeWarrior Project file",
    extensions: &["mcp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967102: FileFormat = FileFormat {
    id: 27_967_102,
    source_type: SourceType::Wikidata,
    name: "Nintendo GameCube/Wii AST",
    extensions: &["ast"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};

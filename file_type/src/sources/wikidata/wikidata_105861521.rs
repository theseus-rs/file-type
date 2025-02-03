use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861521: FileFormat = FileFormat {
    id: 105_861_521,
    source_type: SourceType::Wikidata,
    name: "Lua bytecode (generic)",
    extensions: &["out"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x4C, 0x75, 0x61])],
            },
        }],
    }],
    related_formats: &[],
};

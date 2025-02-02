use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856299: FileFormat = FileFormat {
    id: 105_856_299,
    source_type: SourceType::Wikidata,
    name: "D-LIB bytecode (v2.0)",
    extensions: &["dcf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4C, 0x49, 0x42, 0x32, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};

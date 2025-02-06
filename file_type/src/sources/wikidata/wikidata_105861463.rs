use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861463: FileFormat = FileFormat {
    id: 105_861_463,
    source_type: SourceType::Wikidata,
    name: "Lua 5.3 bytecode",
    extensions: &["out"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x4C, 0x75, 0x61, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};

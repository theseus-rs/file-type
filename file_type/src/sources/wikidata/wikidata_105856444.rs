use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856444: FileFormat = FileFormat {
    id: 105_856_444,
    source_type: SourceType::Wikidata,
    name: "TusiSoft Polyglot dictionary",
    extensions: &["wbf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x54, 0x51, 0x44, 0x4B, 0x4E, 0x49, 0x43, 0x54, 0x53, 0x44, 0x49, 0x43,
                    0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

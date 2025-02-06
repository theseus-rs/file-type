use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861112: FileFormat = FileFormat {
    id: 105_861_112,
    source_type: SourceType::Wikidata,
    name: "Polar SpellChecker dictionary",
    extensions: &["lex"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x6C, 0x61, 0x72, 0x20, 0x53, 0x70, 0x65, 0x6C, 0x6C, 0x43, 0x68,
                    0x65, 0x63, 0x6B, 0x65, 0x72, 0x20, 0x4C, 0x45, 0x58, 0x20, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

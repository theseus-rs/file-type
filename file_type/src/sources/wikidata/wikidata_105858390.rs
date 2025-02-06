use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858390: FileFormat = FileFormat {
    id: 105_858_390,
    source_type: SourceType::Wikidata,
    name: "Ultiboard netlist data",
    extensions: &["ewnet"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x28, 0x00, 0x54, 0x00, 0x6F, 0x00, 0x6F, 0x00, 0x6C, 0x00, 0x49,
                    0x00, 0x6E, 0x00, 0x66, 0x00, 0x6F, 0x00, 0x0D, 0x00, 0x0A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

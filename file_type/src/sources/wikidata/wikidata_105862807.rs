use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862807: FileFormat = FileFormat {
    id: 105_862_807,
    source_type: SourceType::Wikidata,
    name: "Moove Resource Description",
    extensions: &["mpz"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x4F, 0x4F, 0x56, 0x45, 0x5F, 0x50, 0x41, 0x43, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

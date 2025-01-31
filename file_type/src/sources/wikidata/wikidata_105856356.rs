use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856356: FileFormat = FileFormat {
    id: 105_856_356,
    puid: "wikidata/105856356",
    name: "PasswordBox encrypted Database",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x61, 0x73, 0x73, 0x77, 0x6F, 0x72, 0x64, 0x42, 0x6F, 0x78, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

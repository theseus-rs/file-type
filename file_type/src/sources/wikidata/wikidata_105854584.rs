use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854584: FileFormat = FileFormat {
    id: 105_854_584,
    puid: "wikidata/105854584",
    name: "DB/TextWorks Database Access Control",
    extensions: &["acf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x43, 0x46, 0x20, 0x30, 0x30, 0x32, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

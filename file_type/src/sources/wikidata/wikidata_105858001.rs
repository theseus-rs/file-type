use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858001: FileFormat = FileFormat {
    id: 105_858_001,
    puid: "wikidata/105858001",
    name: "DB/TextWorks Database Indexed List",
    extensions: &["ixl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x58, 0x4C, 0x20, 0x30, 0x30, 0x32, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

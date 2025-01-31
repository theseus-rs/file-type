use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852784: FileFormat = FileFormat {
    id: 105_852_784,
    puid: "wikidata/105852784",
    name: "DB/TextWorks Database Deferred Update Directory",
    extensions: &["sdo"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x44, 0x4F, 0x20, 0x30, 0x30, 0x34, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

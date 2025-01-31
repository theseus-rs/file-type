use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861508: FileFormat = FileFormat {
    id: 105_861_508,
    puid: "wikidata/105861508",
    name: "RPG Maker 2000/2003 DataBase",
    extensions: &["ldb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0B, 0x4C, 0x63, 0x66, 0x44, 0x61, 0x74, 0x61, 0x42, 0x61, 0x73, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

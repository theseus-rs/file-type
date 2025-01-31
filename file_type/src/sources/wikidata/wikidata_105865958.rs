use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865958: FileFormat = FileFormat {
    id: 105_865_958,
    puid: "wikidata/105865958",
    name: "BNUPORT Patch Table",
    extensions: &["pat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x4E, 0x55, 0x50, 0x4F, 0x52, 0x54, 0x20, 0x50, 0x61, 0x74, 0x63, 0x68,
                    0x20, 0x54, 0x61, 0x62, 0x6C, 0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

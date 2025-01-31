use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855159: FileFormat = FileFormat {
    id: 105_855_159,
    puid: "wikidata/105855159",
    name: "iGO map",
    extensions: &["fbl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xF9, 0x6D, 0x4A, 0x16, 0x6F, 0xC5, 0x78, 0xEE,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

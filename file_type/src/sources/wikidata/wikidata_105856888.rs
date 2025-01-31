use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856888: FileFormat = FileFormat {
    id: 105_856_888,
    puid: "wikidata/105856888",
    name: "Lotus 1-2-3 Graph",
    extensions: &["gph"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0F, 0x62, 0x6C, 0x64, 0x6E])],
            },
        }],
    }],
    related_formats: &[],
};

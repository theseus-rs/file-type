use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850035: FileFormat = FileFormat {
    id: 105_850_035,
    puid: "wikidata/105850035",
    name: "Color Decision List",
    extensions: &["cdl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x6F, 0x6C, 0x6F, 0x72, 0x44, 0x65, 0x63, 0x69, 0x73, 0x69, 0x6F,
                    0x6E, 0x4C, 0x69, 0x73, 0x74,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

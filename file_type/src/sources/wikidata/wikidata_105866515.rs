use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866515: FileFormat = FileFormat {
    id: 105_866_515,
    puid: "wikidata/105866515",
    name: "CyberLink PowerProducer Project",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x50, 0x50, 0x5F, 0x30, 0x30, 0x30, 0x31, 0x2E, 0x30, 0x30, 0x30, 0x44,
                    0x42, 0x47, 0x4E, 0x56, 0x49, 0x44, 0x45, 0x4F, 0x5F, 0x42, 0x55, 0x52, 0x4E,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

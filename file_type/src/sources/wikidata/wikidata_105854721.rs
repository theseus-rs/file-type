use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854721: FileFormat = FileFormat {
    id: 105_854_721,
    puid: "wikidata/105854721",
    name: "ACCAreader document",
    extensions: &["ar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x12, 0x00, 0x00, 0x00, 0x41, 0x43, 0x43, 0x41, 0x5F, 0x52, 0x45, 0x41, 0x44,
                    0x45, 0x52, 0x5F, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

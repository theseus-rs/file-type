use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856354: FileFormat = FileFormat {
    id: 105_856_354,
    puid: "wikidata/105856354",
    name: "DipTrace PCB",
    extensions: &["dip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x07, 0x44, 0x54, 0x42, 0x4F, 0x41, 0x52, 0x44, 0x0F, 0x42,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

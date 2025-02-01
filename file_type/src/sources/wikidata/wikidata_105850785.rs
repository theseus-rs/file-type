use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850785: FileFormat = FileFormat {
    id: 105_850_785,
    puid: "wikidata/105850785",
    name: "KbdEdit dead table",
    extensions: &["kld"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xFE, 0x44, 0x00, 0x45, 0x00, 0x41, 0x00, 0x44, 0x00, 0x54, 0x00, 0x41,
                    0x00, 0x42, 0x00, 0x4C, 0x00, 0x45, 0x00, 0x0D, 0x00, 0x0A, 0x00, 0x0D, 0x00,
                    0x0A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

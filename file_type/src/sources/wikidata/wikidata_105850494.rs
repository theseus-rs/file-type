use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850494: FileFormat = FileFormat {
    id: 105_850_494,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM aPACK compressed (v1.00)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xBE, 0x0D, 0x01, 0xBF, 0x00, 0x7F, 0x8B, 0xCF, 0xFC, 0x57, 0xF3, 0xA4, 0xC3,
                    0xBF, 0x00, 0x01, 0x57,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850601: FileFormat = FileFormat {
    id: 105_850_601,
    source_type: SourceType::Wikidata,
    name: "Command and Conquer 3 replay",
    extensions: &["cnc3replay"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x26, 0x43, 0x33, 0x20, 0x52, 0x45, 0x50, 0x4C, 0x41, 0x59, 0x20, 0x48,
                    0x45, 0x41, 0x44, 0x45, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

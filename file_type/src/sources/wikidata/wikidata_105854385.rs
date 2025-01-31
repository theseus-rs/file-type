use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854385: FileFormat = FileFormat {
    id: 105_854_385,
    puid: "wikidata/105854385",
    name: "Timex Datalink Wristapp file",
    extensions: &["app"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x57, 0x72, 0x69, 0x73, 0x74, 0x41, 0x70, 0x70, 0x5D, 0x0D, 0x0A, 0x4E,
                    0x61, 0x6D, 0x65, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

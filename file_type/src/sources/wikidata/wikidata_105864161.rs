use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864161: FileFormat = FileFormat {
    id: 105_864_161,
    puid: "wikidata/105864161",
    name: "FMTracker module",
    extensions: &["fmt"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4D, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x01, 0x01, 0x54, 0x68,
                    0x65, 0x20, 0x46, 0x4D, 0x20, 0x54, 0x72, 0x61, 0x63, 0x6B, 0x65, 0x72, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

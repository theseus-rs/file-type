use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857369: FileFormat = FileFormat {
    id: 105_857_369,
    puid: "wikidata/105857369",
    name: "Trackjoy GUS Tracker module",
    extensions: &["joy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x52, 0x41, 0x43, 0x4B, 0x4A, 0x4F, 0x59, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

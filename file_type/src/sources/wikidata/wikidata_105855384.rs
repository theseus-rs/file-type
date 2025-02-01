use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855384: FileFormat = FileFormat {
    id: 105_855_384,
    puid: "wikidata/105855384",
    name: "FMTracker Instruments",
    extensions: &["fmi"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4D, 0x54, 0x49, 0x6E, 0x73, 0x74, 0x72, 0x75, 0x6D, 0x65, 0x6E, 0x74,
                    0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};

use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854331: FileFormat = FileFormat {
    id: 105_854_331,
    puid: "wikidata/105854331",
    name: "Art Of Noise 4-channel module",
    extensions: &["aon"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};

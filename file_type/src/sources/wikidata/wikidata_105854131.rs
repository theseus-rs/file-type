use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854131: FileFormat = FileFormat {
    id: 105_854_131,
    puid: "wikidata/105854131",
    name: "Art Of Noise 8-channel module",
    extensions: &["aon"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4F, 0x4E, 0x38])],
            },
        }],
    }],
    related_formats: &[],
};

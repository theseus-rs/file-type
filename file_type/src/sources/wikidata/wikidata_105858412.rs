use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858412: FileFormat = FileFormat {
    id: 105_858_412,
    puid: "wikidata/105858412",
    name: "EarAche module",
    extensions: &["ea"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x41, 0x53, 0x4F])],
            },
        }],
    }],
    related_formats: &[],
};

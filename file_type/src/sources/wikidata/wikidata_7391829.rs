use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7391829: FileFormat = FileFormat {
    id: 7_391_829,
    puid: "wikidata/7391829",
    name: "Atari SoundHeader",
    extensions: &["sndh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4E, 0x44, 0x48])],
            },
        }],
    }],
    related_formats: &[],
};

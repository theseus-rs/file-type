use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854571: FileFormat = FileFormat {
    id: 105_854_571,
    puid: "wikidata/105854571",
    name: "SoundTool audio",
    extensions: &["snd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x4F, 0x55, 0x4E, 0x44, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};

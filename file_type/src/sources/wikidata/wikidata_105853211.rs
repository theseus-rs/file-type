use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853211: FileFormat = FileFormat {
    id: 105_853_211,
    puid: "wikidata/105853211",
    name: "MPSub subtitles",
    extensions: &["sub"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x54, 0x4C, 0x45, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};

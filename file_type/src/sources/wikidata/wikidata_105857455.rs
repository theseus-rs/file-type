use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857455: FileFormat = FileFormat {
    id: 105_857_455,
    puid: "wikidata/105857455",
    name: "3D Text Animator character",
    extensions: &["anim"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x66, 0x61, 0x63, 0x65, 0x74, 0x20, 0x62, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x0A,
                    0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
